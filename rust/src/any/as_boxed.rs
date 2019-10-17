
#[macro_export]
macro_rules! as_boxed {
    ($trait:tt) => {
    fn clone_boxed(&self) -> Box<dyn $trait>;

    fn to_boxed(self) -> Box<dyn $trait>;
    };

    (impl $trait:tt) => {
    fn clone_boxed(&self) -> Box<dyn $trait> {
        Box::new(self.clone())
    }

    fn to_boxed(self) -> Box<dyn $trait> {
        Box::new(self)
    }
    };
}