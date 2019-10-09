
pub mod any;
pub mod object;
pub mod option;
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
