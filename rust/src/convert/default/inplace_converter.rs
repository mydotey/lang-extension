
use super::*;

#[macro_export]
macro_rules! inplace_converter {
    ($type:ty, $name:ident, $raw_name:ident) => {
lazy_static! {
    pub static ref $name: Box::<InplaceTypeConverter<$type>> =
        new_inplace_type_converter();
    pub static ref $raw_name: Box::<dyn RawTypeConverter> =
        RawTypeConverter::clone_boxed($name.as_ref());
}
    };
}

inplace_converter!(String, STRING_INPLACE_CONVERTER, RAW_STRING_INPLACE_CONVERTER);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::any::*;

    #[test]
    fn inplace_converter_i16() {
        let r = STRING_INPLACE_CONVERTER.convert(&"1".to_string());
        println!("{:?}", r);
        assert_eq!("1", *r.unwrap());
        let r = RAW_STRING_INPLACE_CONVERTER.convert_raw(&"1".to_string());
        println!("{:?}", r);
        assert_eq!(&Value::to_boxed("1".to_string()), &r.unwrap());

        let raw_t = STRING_INPLACE_CONVERTER.clone();
        let raw_t2 = STRING_INPLACE_CONVERTER.clone();
        assert_eq!(&raw_t, &raw_t2);
        let raw_t = RAW_STRING_INPLACE_CONVERTER.clone();
        let raw_t2 = RAW_STRING_INPLACE_CONVERTER.clone();
        assert_eq!(&raw_t, &raw_t2);
    }
}