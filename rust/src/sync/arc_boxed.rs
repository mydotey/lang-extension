
#[macro_export]
macro_rules! arc_boxed {
    ($value:tt) => {
        std::sync::Arc::new(Box::new($value))
    };
    ($value:path) => {
        std::sync::Arc::new(Box::new($value))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::fmt::*;

    #[test]
    fn arc_boxed() {
        let x = arc_boxed!(10);
        println!("{}", x);
        let y = arc_boxed!(crate::sync::arc_boxed::tests::arc_boxed);
        println!("{:?}", y.to_instance_string());
    }
}
