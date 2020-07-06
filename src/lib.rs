mod template;
mod render;

use rutie::{class, methods, AnyObject, Module, Object, RString, VM, Class, Exception, NilClass, types::Value, AnyException};
use template::Template;
use render::render_template;

class!(Liquidy);

methods!(
    Liquidy,
    _itself,
    fn pub_parse(input: RString) -> AnyObject {
        let input = input.map_err(|e| VM::raise_ex(e)).unwrap();
        let input = input.to_string();

        let mut template = Template::new();
        template.emit(input);

        AnyObject::from(&template)
    }

    fn pub_render(template: AnyObject) -> AnyObject {
        let template: AnyObject = template.map_err(|e| VM::raise_ex(e)).unwrap();
        let template: Value = template.into();
        let template = Template::from(template);

        match render_template(&template) {
            Ok(result) => RString::from(result).into(),
            Err(_e) => {
                VM::raise_ex(rutie::AnyException::new("RenderError", Some("could not render template")));
                NilClass::new().into()
            }
        }
    }
);

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
          VM::raise_ex(AnyException::new("InvalidTemplateError", Some("could not serialize template")));
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
          VM::raise_ex(AnyException::new("InvalidTemplateError", Some("could not parse template")));
          NilClass::new().into()
      }
    }
  }
}
