use std::fmt;
use std::any::type_name;
use std::sync::Arc;

use crate::raw_type_converter;
use super::*;
use crate::ops::function::*;

#[derive(Clone)]
pub struct DefaultTypeConverter<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> {
    convert: Arc::<FunctionRef<S, Result<Box<T>, Box<dyn Value>>>>
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> DefaultTypeConverter<S, T> {
    pub fn new(convert: FunctionRef<S, Result<Box<T>, Box<dyn Value>>>) -> Self {
        Self {
            convert: Arc::new(convert)
        }
    }
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> TypeConverter<S, T>
    for DefaultTypeConverter<S, T> {

    fn convert(&self, source: &S) -> Result<Box<T>, Box<dyn Value>> {
        self.convert.as_ref()(source)
    }

}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> PartialEq for DefaultTypeConverter<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.convert.as_ref().reference_equals(other.convert.as_ref())
    }
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> Eq for DefaultTypeConverter<S, T> { }

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> fmt::Debug for DefaultTypeConverter<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ source_type: {}, target_type: {}, converter: {}", type_name::<S>(), type_name::<T>(),
            type_name::<FunctionRef<S, Result<T, Box<dyn Value>>>>())
    }
}

raw_type_converter!(DefaultTypeConverter; S; T);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_converter() {
        let converter = DefaultTypeConverter::<i32, String>::new(Box::new(|v|{
            Ok(Box::new(v.to_string()))
        }));
        println!("converter: {:?}", converter);
        let s = TypeConverter::convert(&converter, &10).unwrap();
        println!("{:?}", s);
        assert_eq!("10".to_string(), *s);

        assert_eq!(converter, converter.clone());
    }
}