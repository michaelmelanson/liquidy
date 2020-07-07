mod context;
mod template;

mod compiler;
mod parser;
mod render;

use compiler::compile_intermediate;
use context::Context;
use parser::parse_template;
use render::render_template;
use rutie::{
    class, methods,
    rubysys::class::rb_scan_args,
    types::{Argc, Value},
    util::str_to_cstring,
    AnyException, AnyObject, Array, Class, Exception, Module, NilClass, Object, RString, VM,
};
use std::mem;
use template::Template;

class!(Liquidy);

methods!(
    Liquidy,
    _itself,
    fn pub_parse(input: RString) -> AnyObject {
        let input = input.map_err(|e| VM::raise_ex(e)).unwrap();
        let input = input.to_string();

        match parse_template(input) {
            Ok(intermediate) => match compile_intermediate(&intermediate) {
                Ok(template) => AnyObject::from(&template),
                Err(err) => {
                    VM::raise_ex(AnyException::new(
                        "CompileError",
                        Some(&format!("{:?}", err)),
                    ));
                    NilClass::new().into()
                }
            },
            Err(err) => {
                VM::raise_ex(AnyException::new(
                    "CompileError",
                    Some(&format!("{:?}", err)),
                ));
                NilClass::new().into()
            }
        }
    }
);

extern "C" fn pub_render(argc: Argc, argv: *const AnyObject, _: AnyObject) -> AnyObject {
    let args = Value::from(0);

    unsafe {
        let p_argv: *const Value = mem::transmute(argv);
        rb_scan_args(argc, p_argv, str_to_cstring("*").as_ptr(), &args)
    };

    let arguments = Array::from(args);

    if argc < 1 {
        VM::raise_ex(AnyException::new(
            "ArgumentError",
            Some("not enough arguments to render (1 required)"),
        ));
    }

    let template: Value = arguments.at(0).into();
    let template = Template::from(template);

    let context = arguments.at(1);
    // println!("Context is {:?}", unsafe { context.send("inspect", &[]) }.try_convert_to::<RString>().map(|s| s.to_string()));
    let context = Context::new(context);

    match render_template(&template, &context) {
        Ok(result) => RString::from(result).into(),
        Err(_e) => {
            VM::raise_ex(AnyException::new(
                "RenderError",
                Some("could not render template"),
            ));
            NilClass::new().into()
        }
    }
}

#[no_mangle]
pub extern "C" fn init_liquidy() {
    Module::from_existing("Liquidy").define(|module| {
        module.def_self("parse", pub_parse);
        module.def_self("render", pub_render);

        module.define_nested_class("Template", None);

        let standard_error = Class::from_existing("StandardError");
        module.define_nested_class("InvalidTemplateError", Some(&standard_error));
        module.define_nested_class("RenderError", Some(&standard_error));
    });
}

impl From<Value> for Template {
    fn from(value: Value) -> Self {
        let json: RString = value.into();
        let json = json.to_str();

        serde_json::from_str(json)
            .map_err(|_e| {
                VM::raise_ex(AnyException::new(
                    "InvalidTemplateError",
                    Some("could not serialize template"),
                ));
            })
            .unwrap()
    }
}

impl Object for Template {
    #[inline]
    fn value(&self) -> Value {
        match serde_json::to_string(self) {
            Ok(json) => RString::from(json).into(),
            Err(_e) => {
                VM::raise_ex(AnyException::new(
                    "InvalidTemplateError",
                    Some("could not parse template"),
                ));
                NilClass::new().into()
            }
        }
    }
}
