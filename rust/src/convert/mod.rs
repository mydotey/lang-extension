use std::any::TypeId;
use crate::any::*;
use crate::*;

pub mod default;

pub trait RawTypeConverter: Value + Sync + Send {

    fn get_source_type(&self) -> TypeId;

    fn get_target_type(&self) -> TypeId;

    fn convert(&self, source: &dyn Value) -> Result<Box<dyn Value>, Box<dyn Value>>;

as_trait!(RawTypeConverter);
as_boxed!(RawTypeConverter);
}

pub trait TypeConverter<S: ?Sized + Value, T: ?Sized + Value> : RawTypeConverter {

    fn convert(&self, source: &S) -> Result<Box<T>, Box<dyn Value>>;

}

#[macro_export]
macro_rules! raw_type_converter {
    (inner $type: ty, $source_type: ty, $target_type: ty) => {
    fn get_source_type(&self) -> TypeId {
        TypeId::of::<$source_type>()
    }

    fn get_target_type(&self) -> TypeId {
        TypeId::of::<$target_type>()
    }

    fn convert(&self, source: &dyn Value) -> Result<Box<dyn Value>, Box<dyn Value>> {
        match source.as_any_ref().downcast_ref::<$source_type>() {
            Some(s) => {
                match TypeConverter::convert(self, s) {
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
impl RawTypeConverter for $type {
raw_type_converter!(inner $type, $source_type, $target_type);
}

unsafe impl Sync for $type { }
unsafe impl Send for $type { }

    };

    ($generic_type: tt; $source_type: tt; $target_type: tt) => {
impl<$source_type: ?Sized + ValueConstraint, $target_type: ?Sized + ValueConstraint>
    RawTypeConverter for $generic_type<$source_type, $target_type> {
raw_type_converter!(inner $generic_type, $source_type, $target_type);
}

unsafe impl<$source_type: ?Sized + ValueConstraint, $target_type: ?Sized + ValueConstraint> Sync
    for $generic_type<$source_type, $target_type> { }
unsafe impl<$source_type: ?Sized + ValueConstraint, $target_type: ?Sized + ValueConstraint>
    Send for $generic_type<$source_type, $target_type> { }

    };
}

boxed_value_trait!(RawTypeConverter);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, PartialEq, Eq, Debug, Clone)]
    struct C;

    impl TypeConverter<i32, String> for C {

        fn convert(&self, source: &i32) -> Result<Box<String>, Box<dyn Value>> {
            Ok(Box::new(source.to_string()))
        }

    }

    raw_type_converter!(C, i32, String);

    #[test]
    fn type_convert() {
        match TypeConverter::convert(&C, &9) {
            Ok(s) => {
                println!("{}", s);
                assert_eq!("9".to_string(), *s);
            },
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }

        match RawTypeConverter::convert(RawTypeConverter::as_trait_ref(&C{}), Value::as_trait_ref(&9)) {
            Ok(s) => {
                println!("{:?}", s);
                assert_eq!("9".to_string(), *s.as_ref().as_any_ref().downcast_ref::<String>().unwrap());
            },
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }

        match RawTypeConverter::convert(RawTypeConverter::as_trait_ref(&C{}), Value::as_trait_ref(&"ok")) {
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