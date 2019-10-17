
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

#[macro_export]
macro_rules! as_trait {
    ($trait: tt) => {
    fn as_trait_ref(&self) -> &dyn $trait;

    fn as_trait_mut(&mut self) -> &mut dyn $trait;
    };

    (impl $trait: tt) => {
    fn as_trait_ref(&self) -> &dyn $trait {
        self
    }

    fn as_trait_mut(&mut self) -> &mut dyn $trait {
        self
    }
    };
}
