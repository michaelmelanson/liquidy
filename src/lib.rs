use rutie::{class, methods, AnyObject, Class, Object, RString, VM, Fixnum};

class!(LiquidyVM);

methods!(
    LiquidyVM,
    _itself,
    fn pub_evaluate(input: RString) -> AnyObject {
        let input = input.map_err(|e| VM::raise_ex(e)).unwrap();
        let input = input.to_string();

        println!("Rust: evaluating {:?}", input);
        let result = VM::eval(&input);
        println!("Rust: result is {:?}", result);
        let result = result.map_err(|e| VM::raise_ex(e)).unwrap();
        println!("Rust:    ... as string: {:?}", result.try_convert_to::<RString>().map(|value| value.to_string()));
        println!("Rust:    ... as number: {:?}", result.try_convert_to::<Fixnum>().map(|value| value.to_i64()));

        result
    }
);

#[no_mangle]
pub extern "C" fn init_liquidy() {
    Class::new("LiquidyVM", None).define(|itself| {
        itself.def_self("evaluate", pub_evaluate);
    });
}
