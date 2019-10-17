
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
