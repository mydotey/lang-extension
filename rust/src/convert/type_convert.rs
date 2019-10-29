use std::any::{ TypeId, type_name };
use std::sync::Arc;
use std::fmt;

use crate::*;
use crate::any::*;
use crate::ops::function::*;

pub trait RawTypeConverter: Value + Sync + Send {

    fn get_source_type(&self) -> TypeId;

    fn get_target_type(&self) -> TypeId;

    fn convert_raw(&self, source: &dyn Value) -> Result<Box<dyn Value>, Box<dyn Value>>;

as_trait!(RawTypeConverter);
as_boxed!(RawTypeConverter);
}

boxed_value_trait!(RawTypeConverter);

pub trait TypeConverter<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> : RawTypeConverter {

    fn convert(&self, source: &S) -> Result<Box<T>, Box<dyn Value>>;

as_boxed!(TypeConverter<S, T>);
}

boxed_value_trait!(TypeConverter<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint>);

#[macro_export]
macro_rules! raw_type_converter {
    ($source_type: ty, $target_type: ty) => {
    fn get_source_type(&self) -> TypeId {
        TypeId::of::<$source_type>()
    }

    fn get_target_type(&self) -> TypeId {
        TypeId::of::<$target_type>()
    }

    fn convert_raw(&self, source: &dyn Value) -> Result<Box<dyn Value>, Box<dyn Value>> {
        match source.as_any_ref().downcast_ref::<$source_type>() {
            Some(s) => {
                match $crate::convert::TypeConverter::convert(self, s) {
                    Ok(t) => Ok(Value::to_boxed(*t)),
                    Err(err) => Err(err)
                }
            },
            None => Err(Box::new(format!("source value {:?} is of an unsupported type: {:?}, only support: {:?}",
                source, source.type_name(), std::any::type_name::<$source_type>())))
        }
    }

as_trait!(impl RawTypeConverter);
as_boxed!(impl RawTypeConverter);
    };

    ($type: ty, $source_type: ty, $target_type: ty) => {
impl $crate::convert::RawTypeConverter for $type {
raw_type_converter!($source_type, $target_type);
}

unsafe impl Sync for $type { }
unsafe impl Send for $type { }

    };

    ($type: tt<$source_type:tt, $target_type:tt>) => {
impl<$source_type: ?Sized + ValueConstraint, $target_type: ?Sized + ValueConstraint>
    $crate::convert::RawTypeConverter for $type<$source_type, $target_type> {
raw_type_converter!($source_type, $target_type);
}

unsafe impl<$source_type: ?Sized + ValueConstraint, $target_type: ?Sized + ValueConstraint> Sync
    for $type<$source_type, $target_type> { }
unsafe impl<$source_type: ?Sized + ValueConstraint, $target_type: ?Sized + ValueConstraint>
    Send for $type<$source_type, $target_type> { }

    };
}

#[derive(Clone)]
pub struct DefaultTypeConverter<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> {
    convert: FunctionRef<S, Result<Box<T>, Box<dyn Value>>>
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> DefaultTypeConverter<S, T> {
    pub fn new(convert: Box<dyn Fn(&S) -> Result<Box<T>, Box<dyn Value>>>) -> Self {
        Self::wrap(Arc::new(convert))
    }

    pub fn wrap(convert: FunctionRef<S, Result<Box<T>, Box<dyn Value>>>) -> Self {
        Self {
            convert: convert
        }
    }
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> TypeConverter<S, T>
    for DefaultTypeConverter<S, T> {

    fn convert(&self, source: &S) -> Result<Box<T>, Box<dyn Value>> {
        self.convert.as_ref()(source)
    }

as_boxed!(impl TypeConverter<S, T>);
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> PartialEq for DefaultTypeConverter<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.convert.as_ref().reference_equals(other.convert.as_ref())
    }
}

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> Eq for DefaultTypeConverter<S, T> { }

impl<S: ?Sized + ValueConstraint, T: ?Sized + ValueConstraint> fmt::Debug for DefaultTypeConverter<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {{ source_type: {}, target_type: {}, converter: {}",
            type_name::<DefaultTypeConverter<S, T>>(),
            type_name::<S>(), type_name::<T>(),
            type_name::<FunctionRef<S, Result<T, Box<dyn Value>>>>())
    }
}

raw_type_converter!(DefaultTypeConverter<S, T>);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, PartialEq, Eq, Debug, Clone)]
    struct C;

    impl TypeConverter<i32, String> for C {

        fn convert(&self, source: &i32) -> Result<Box<String>, Box<dyn Value>> {
            Ok(Box::new(source.to_string()))
        }

    as_boxed!(impl TypeConverter<i32, String>);
    }

    raw_type_converter!(C, i32, String);

    #[test]
    fn type_convert() {
        match C.convert(&9) {
            Ok(s) => {
                println!("{}", s);
                assert_eq!("9".to_string(), *s);
            },
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }

        match C.convert_raw(Value::as_trait_ref(&9)) {
            Ok(s) => {
                println!("{:?}", s);
                assert_eq!("9".to_string(), *s.as_ref().as_any_ref().downcast_ref::<String>().unwrap());
            },
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }

        match C.convert_raw(Value::as_trait_ref(&"ok")) {
            Ok(s) => {
                println!("{:?}", s);
                assert!(false);
            },
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}