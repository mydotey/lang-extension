use std::hash::{ Hash, Hasher };
use std::fmt;
use std::any::type_name;
use std::sync::Arc;

use super::*;
use crate::*;
use crate::ops::function::*;

#[derive(Clone)]
pub struct DefaultTypeConverter<S: ObjectConstraits, T: ObjectConstraits> {
    convert: Arc::<FunctionRef<S, Result<T, Box<dyn Object>>>>
}

impl<S: ObjectConstraits, T: ObjectConstraits> DefaultTypeConverter<S, T> {
    pub fn new(convert: FunctionRef<S, Result<T, Box<dyn Object>>>) -> Self {
        Self {
            convert: Arc::new(convert)
        }
    }
}

impl<S: ObjectConstraits, T: ObjectConstraits> TypeConverter<S, T> for DefaultTypeConverter<S, T> {

    fn convert(&self, source: &S) -> Result<T, Box<dyn Object>> {
        self.convert.as_ref()(source)
    }

    fn as_raw(&self) -> &dyn RawTypeConverter {
        self
    }

}

impl<S: ObjectConstraits, T: ObjectConstraits> Hash for DefaultTypeConverter<S, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.convert.as_ref().memory_address());
    }
}

impl<S: ObjectConstraits, T: ObjectConstraits> PartialEq for DefaultTypeConverter<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.convert.as_ref().reference_equals(other.convert.as_ref())
    }
}

impl<S: ObjectConstraits, T: ObjectConstraits> Eq for DefaultTypeConverter<S, T> { }

impl<S: ObjectConstraits, T: ObjectConstraits> fmt::Debug for DefaultTypeConverter<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ source_type: {}, target_type: {}, converter: {}", type_name::<S>(), type_name::<T>(),
            type_name::<FunctionRef<S, Result<T, Box<dyn Object>>>>())
    }
}

raw_type_converter!(DefaultTypeConverter; S; T);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_converter() {
        let converter = DefaultTypeConverter::<i32, String>::new(Box::new(|v|{
            Ok(v.to_string())
        }));
        println!("converter: {:?}", converter);
        let s = TypeConverter::convert(&converter, &10);
        println!("{:?}", s.unwrap());

        assert_eq!(converter, converter.clone());
    }
}