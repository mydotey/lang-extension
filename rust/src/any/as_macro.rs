
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

    (impl Hash for $trait:tt) => {
impl std::hash::Hash for Box<dyn $trait> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}
    };

    (impl PartialEq for $trait:tt) => {
impl PartialEq for Box<dyn $trait> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any_ref())
    }
}
    };

    (impl Eq for $trait:tt) => {
impl Eq for Box<dyn $trait> { }
    };

    (impl Clone for $trait:tt) => {
impl Clone for Box<dyn $trait> {
    fn clone(&self) -> Self {
        $trait::clone_boxed(self.as_ref())
    }
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
