use super::*;

#[macro_export]
macro_rules! string_to_primitive_converter {
    ($type:ty, $name:ident, $raw_name:ident) => {
        pub static $name: std::sync::LazyLock<Box<dyn TypeConverter<String, $type>>> =
            std::sync::LazyLock::new(|| {
                TypeConverter::to_boxed(DefaultTypeConverter::<String, $type>::new(Box::new(|s| {
                    match s.parse::<$type>() {
                        Ok(v) => Ok(Box::new(v)),
                        Err(e) => Err(Box::new(e.to_string())),
                    }
                })))
            });

        pub static $raw_name: std::sync::LazyLock<Box<dyn RawTypeConverter>> =
            std::sync::LazyLock::new(|| RawTypeConverter::clone_boxed($name.as_ref()));
    };
}

string_to_primitive_converter!(bool, STRING_TO_BOOL_CONVERTER, RAW_STRING_TO_BOOL_CONVERTER);
string_to_primitive_converter!(i8, STRING_TO_I8_CONVERTER, RAW_STRING_TO_I8_CONVERTER);
string_to_primitive_converter!(i16, STRING_TO_I16_CONVERTER, RAW_STRING_TO_I16_CONVERTER);
string_to_primitive_converter!(i32, STRING_TO_I32_CONVERTER, RAW_STRING_TO_I32_CONVERTER);
string_to_primitive_converter!(i64, STRING_TO_I64_CONVERTER, RAW_STRING_TO_I64_CONVERTER);
string_to_primitive_converter!(i128, STRING_TO_I128_CONVERTER, RAW_STRING_TO_I128_CONVERTER);
string_to_primitive_converter!(
    isize,
    STRING_TO_ISIZE_CONVERTER,
    RAW_STRING_TO_ISIZE_CONVERTER
);
string_to_primitive_converter!(u8, STRING_TO_U8_CONVERTER, RAW_STRING_TO_U8_CONVERTER);
string_to_primitive_converter!(u16, STRING_TO_U16_CONVERTER, RAW_STRING_TO_U16_CONVERTER);
string_to_primitive_converter!(u32, STRING_TO_U32_CONVERTER, RAW_STRING_TO_U32_CONVERTER);
string_to_primitive_converter!(u64, STRING_TO_U64_CONVERTER, RAW_STRING_TO_U64_CONVERTER);
string_to_primitive_converter!(u128, STRING_TO_U128_CONVERTER, RAW_STRING_TO_U128_CONVERTER);
string_to_primitive_converter!(
    usize,
    STRING_TO_USIZE_CONVERTER,
    RAW_STRING_TO_USIZE_CONVERTER
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converter_test() {
        let r = STRING_TO_I32_CONVERTER.convert(&"1".to_string());
        println!("{:?}", r);
        assert_eq!(Ok(Box::new(1)), r);
        let r = STRING_TO_I32_CONVERTER.convert(&"x1".to_string());
        println!("{:?}", r);
        assert!(r.is_err());
        let raw_t = RAW_STRING_TO_I32_CONVERTER.clone();
        let raw_t2 = RAW_STRING_TO_I32_CONVERTER.clone();
        assert_eq!(&raw_t, &raw_t2);
    }
}
