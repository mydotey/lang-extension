
#[macro_use]
extern crate lazy_static;

pub mod any;
pub mod fmt;
pub mod ops;
pub mod convert;
pub mod sync;

#[cfg(test)]
mod tests {
#[macro_export]
macro_rules! test_print {
    ($($trait:tt<$literal0:literal $(+ $param:literal)+>),*) => {
        {
            println!("{}", stringify!($($literal0 $(+ $param)+),*));
        }
    };
}

    #[test]
    fn macro_repeat() {
        test_print!(me<1 + 2 + 3>, to<1 + 2 + 3>);
    }
}
