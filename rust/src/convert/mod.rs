use std::any::TypeId;
use crate::object::*;

pub mod default;

pub trait RawTypeConverter: Object + Sync + Send {

    fn get_source_type(&self) -> TypeId;

    fn get_target_type(&self) -> TypeId;

    fn convert(&self, source: &dyn Object) -> Result<Box<dyn Object>, Box<dyn Object>>;

}

pub trait TypeConverter<S: ObjectConstraits, T: ObjectConstraits> : RawTypeConverter {

    fn convert(&self, source: &S) -> Result<T, Box<dyn Object>>;

    fn as_raw(&self) -> &dyn RawTypeConverter;

}

#[allow(unused_macros)]
#[macro_export]
macro_rules! raw_type_converter {
    ($type: ty, $source_type: ty, $target_type: ty) => {
impl RawTypeConverter for $type {

    fn get_source_type(&self) -> TypeId {
        TypeId::of::<$source_type>()
    }

    fn get_target_type(&self) -> TypeId {
        TypeId::of::<$target_type>()
    }

    fn convert(&self, source: &dyn Object) -> Result<Box<dyn Object>, Box<dyn Object>> {
        match source.as_any().downcast_ref::<$source_type>() {
            Some(s) => {
                match TypeConverter::convert(self, s) {
                    Ok(t) => Ok(Box::new(t)),
                    Err(err) => Err(err)
                }
            },
            None => Err(Box::new(format!("source object {} is of an unsupported type: {:?}, only support: {:?}",
                source.to_debug_string(), source.type_name(), std::any::type_name::<$source_type>())))
        }
    }

}

unsafe impl Sync for $type { }
unsafe impl Send for $type { }

    };
    ($type: tt; $source_type: tt; $target_type: tt) => {
impl<$source_type: ObjectConstraits, $target_type: ObjectConstraits>
    RawTypeConverter for $type<$source_type, $target_type> {

    fn get_source_type(&self) -> TypeId {
        TypeId::of::<$source_type>()
    }

    fn get_target_type(&self) -> TypeId {
        TypeId::of::<$target_type>()
    }

    fn convert(&self, source: &dyn Object) -> Result<Box<dyn Object>, Box<dyn Object>> {
        match source.as_any().downcast_ref::<$source_type>() {
            Some(s) => {
                match TypeConverter::convert(self, s) {
                    Ok(t) => Ok(Box::new(t)),
                    Err(err) => Err(err)
                }
            },
            None => Err(Box::new(format!("source object {} is of an unsupported type: {:?}, only support: {:?}",
                source.to_debug_string(), source.type_name(),
                std::any::type_name::<$type::<$source_type, $target_type>>())))
        }
    }

}

unsafe impl<$source_type: ObjectConstraits, $target_type: ObjectConstraits> Sync
    for $type<$source_type, $target_type> { }
unsafe impl<$source_type: ObjectConstraits, $target_type: ObjectConstraits>
    Send for $type<$source_type, $target_type> { }

    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, PartialEq, Eq, Debug, Clone)]
    struct C;

    impl TypeConverter<i32, String> for C {

        fn convert(&self, source: &i32) -> Result<String, Box<dyn Object>> {
            Ok(source.to_string())
        }

        fn as_raw(&self) -> &dyn RawTypeConverter {
            self
        }

    }

    raw_type_converter!(C, i32, String);

    #[test]
    fn type_convert() {
        match TypeConverter::convert(&C, &9) {
            Ok(s) => println!("{}", s),
            Err(err) => println!("{:?}", err)
        }

        match RawTypeConverter::convert(C.as_raw(), 9.as_object()) {
            Ok(s) => println!("{:?}", s),
            Err(err) => println!("{:?}", err)
        }

        match RawTypeConverter::convert(C.as_raw(), "ok".as_object()) {
            Ok(s) => println!("{:?}", s),
            Err(err) => println!("{:?}", err)
        }
    }
}