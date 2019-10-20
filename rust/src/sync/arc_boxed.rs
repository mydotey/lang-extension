
#[macro_export]
macro_rules! arc_boxed {
    ($value:tt) => {
        std::sync::Arc::new(Box::new($value))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn arc_boxed() {
        let x = arc_boxed!(10);
        println!("{}", x);
    }
}
