use crate::template::Keypath;
use rutie::{AnyObject, Hash, Object, RString, Symbol};

pub struct Context {
    object: AnyObject,
}

impl Context {
    pub fn new(object: AnyObject) -> Self {
        Context { object }
    }

    pub fn keypath(&self, path: &Keypath) -> Option<String> {
        let mut cursor: AnyObject = self.object.clone();
        let parts = path.split(".");

        for part in parts {
            if let Ok(hash) = cursor.try_convert_to::<Hash>() {
                cursor = hash.at(&Symbol::new(&part.to_string())).clone();
            } else {
                return None;
            }
        }

        if let Ok(value) = cursor.try_convert_to::<RString>() {
            Some(value.to_string())
        } else {
            None
        }
    }
}
