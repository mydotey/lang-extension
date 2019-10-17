
pub mod any;

pub mod fmt;
pub mod ops;

#[macro_use]
pub mod convert;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
