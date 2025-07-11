#![allow(dead_code)]
#[macro_export]
macro_rules! as_boxed {
    ($trait:tt) => {
    fn clone_boxed(&self) -> Box<dyn $trait>;

    fn to_boxed(self) -> Box<dyn $trait>;
    };

    ($trait:tt<$($param:tt), *>) => {
    fn clone_boxed(&self) -> Box<dyn $trait<$($param), *>>;

    fn to_boxed(self) -> Box<dyn $trait<$($param), *>>;
    };

    (impl $trait:tt) => {
    fn clone_boxed(&self) -> Box<dyn $trait> {
        Box::new(self.clone())
    }

    fn to_boxed(self) -> Box<dyn $trait> {
        Box::new(self)
    }
    };

    (impl $trait:tt<$($param:tt), *>) => {
    fn clone_boxed(&self) -> Box<dyn $trait<$($param), *>> {
        Box::new(self.clone())
    }

    fn to_boxed(self) -> Box<dyn $trait<$($param), *>> {
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

    (impl Hash for $trait:tt<$($param:tt), *>) => {
impl<$($param), *> std::hash::Hash for Box<dyn $trait<$($param), *>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}
    };

    (impl Hash for $trait:tt<$($param:tt: $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: $constraint0 $(+ $constraint)*), *> std::hash::Hash for Box<dyn $trait<$($param), *>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}
    };

    (impl Hash for $trait:tt<$($param:tt: ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: ?Sized + $constraint0 $(+ $constraint)*), *> std::hash::Hash for Box<dyn $trait<$($param), *>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}
    };

    (impl Hash for $trait:tt<$($param:tt: 'static + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + $constraint0 $(+ $constraint)*), *> std::hash::Hash for Box<dyn $trait<$($param), *>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}
    };

    (impl Hash for $trait:tt<$($param:tt: 'static + ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + ?Sized + $constraint0 $(+ $constraint)*), *> std::hash::Hash for Box<dyn $trait<$($param), *>> {
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

    (impl PartialEq for $trait:tt<$($param:tt), *>) => {
impl<$($param), *> PartialEq for Box<dyn $trait<$($param), *>> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any_ref())
    }
}
    };

    (impl PartialEq for $trait:tt<$($param:tt: $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: $constraint0 $(+ $constraint)*), *> PartialEq for Box<dyn $trait<$($param), *>> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any_ref())
    }
}
    };

    (impl PartialEq for $trait:tt<$($param:tt: ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: ?Sized + $constraint0 $(+ $constraint)*), *> PartialEq for Box<dyn $trait<$($param), *>> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any_ref())
    }
}
    };

    (impl PartialEq for $trait:tt<$($param:tt: 'static + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + $constraint0 $(+ $constraint)*), *> PartialEq for Box<dyn $trait<$($param), *>> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any_ref())
    }
}
    };

    (impl PartialEq for $trait:tt<$($param:tt: 'static + ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + ?Sized + $constraint0 $(+ $constraint)*), *> PartialEq for Box<dyn $trait<$($param), *>> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any_ref())
    }
}
    };

    (impl Eq for $trait:tt) => {
impl Eq for Box<dyn $trait> { }
    };

    (impl Eq for $trait:tt<$($param:tt), *>) => {
impl<$($param), *> Eq for Box<dyn $trait<$($param), *>> { }
    };

    (impl Eq for $trait:tt<$($param:tt: $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: $constraint0 $(+ $constraint)*), *> Eq for Box<dyn $trait<$($param), *>> { }
    };

    (impl Eq for $trait:tt<$($param:tt: ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: ?Sized + $constraint0 $(+ $constraint)*), *> Eq for Box<dyn $trait<$($param), *>> { }
    };

    (impl Eq for $trait:tt<$($param:tt: 'static + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + $constraint0 $(+ $constraint)*), *> Eq for Box<dyn $trait<$($param), *>> { }
    };

    (impl Eq for $trait:tt<$($param:tt: 'static + ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + ?Sized + $constraint0 $(+ $constraint)*), *> Eq for Box<dyn $trait<$($param), *>> { }
    };

    (impl Clone for $trait:tt) => {
impl Clone for Box<dyn $trait> {
    fn clone(&self) -> Self {
        $trait::clone_boxed(self.as_ref())
    }
}
    };

    (impl Clone for $trait:tt<$($param:tt), *>) => {
impl<$($param), *> Clone for Box<dyn $trait<$($param), *>> {
    fn clone(&self) -> Self {
        $trait::clone_boxed(self.as_ref())
    }
}
    };

    (impl Clone for $trait:tt<$($param:tt: $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: $constraint0 $(+ $constraint)*), *> Clone for Box<dyn $trait<$($param), *>> {
    fn clone(&self) -> Self {
        $trait::clone_boxed(self.as_ref())
    }
}
    };

    (impl Clone for $trait:tt<$($param:tt: ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: ?Sized + $constraint0 $(+ $constraint)*), *> Clone for Box<dyn $trait<$($param), *>> {
    fn clone(&self) -> Self {
        $trait::clone_boxed(self.as_ref())
    }
}
    };

    (impl Clone for $trait:tt<$($param:tt: 'static + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + $constraint0 $(+ $constraint)*), *> Clone for Box<dyn $trait<$($param), *>> {
    fn clone(&self) -> Self {
        $trait::clone_boxed(self.as_ref())
    }
}
    };

    (impl Clone for $trait:tt<$($param:tt: 'static + ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
impl<$($param: 'static + ?Sized + $constraint0 $(+ $constraint)*), *> Clone for Box<dyn $trait<$($param), *>> {
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

    ($trait:tt<$($param:tt), *>) => {
    fn as_trait_ref(&self) -> &dyn $trait<$($param), *>;

    fn as_trait_mut(&mut self) -> &mut dyn $trait<$($param), *>;
    };

    (impl $trait: tt) => {
    fn as_trait_ref(&self) -> &dyn $trait {
        self
    }

    fn as_trait_mut(&mut self) -> &mut dyn $trait {
        self
    }
    };

    (impl $trait:tt<$($param:tt), *>) => {
    fn as_trait_ref(&self) -> &dyn $trait<$($param), *> {
        self
    }

    fn as_trait_mut(&mut self) -> &mut dyn $trait<$($param), *> {
        self
    }
    };
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::any::*;
    use crate::*;
    use std::fmt::Debug;
    use std::marker::*;

    trait SomeType: Key {
        fn say(&self);

        as_boxed!(SomeType);
        as_trait!(SomeType);
    }

    as_boxed!(impl Hash for SomeType);
    as_boxed!(impl PartialEq for SomeType);
    as_boxed!(impl Eq for SomeType);
    as_boxed!(impl Clone for SomeType);

    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct ST {}

    impl SomeType for ST {
        fn say(&self) {}

        as_boxed!(impl SomeType);
        as_trait!(impl SomeType);
    }

    trait SomeType1<K>: Key {
        fn say(&self, k: K);

        as_boxed!(SomeType1<K>);
        as_trait!(SomeType1<K>);
    }

    as_boxed!(impl Hash for SomeType1<K: 'static>);
    as_boxed!(impl PartialEq for SomeType1<K: 'static>);
    as_boxed!(impl Eq for SomeType1<K: 'static>);
    as_boxed!(impl Clone for SomeType1<K: 'static>);

    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct ST1<K: KeyConstraint> {
        k: PhantomData<K>,
    }

    impl<K: KeyConstraint> SomeType1<K> for ST1<K> {
        fn say(&self, _k: K) {}

        as_boxed!(impl SomeType1<K>);
        as_trait!(impl SomeType1<K>);
    }

    trait SomeType2<K, V>: Key {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType2<K, V>);
        as_trait!(SomeType2<K, V>);
    }

    as_boxed!(impl Hash for SomeType2<K: 'static, V: 'static>);
    as_boxed!(impl PartialEq for SomeType2<K: 'static, V: 'static>);
    as_boxed!(impl Eq for SomeType2<K: 'static, V: 'static>);
    as_boxed!(impl Clone for SomeType2<K: 'static, V: 'static>);

    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct ST2<K: KeyConstraint, V: ValueConstraint> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: KeyConstraint, V: KeyConstraint> SomeType2<K, V> for ST2<K, V> {
        fn say(&self, _k: K, _v: V) {}

        as_boxed!(impl SomeType2<K, V>);
        as_trait!(impl SomeType2<K, V>);
    }

    trait SomeType3<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>: Key {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType3<K, V>);
        as_trait!(SomeType3<K, V>);
    }

    as_boxed!(impl Hash for SomeType3<K: KeyConstraint, V: ValueConstraint>);
    as_boxed!(impl PartialEq for SomeType3<K: KeyConstraint, V: ValueConstraint>);
    as_boxed!(impl Eq for SomeType3<K: KeyConstraint, V: ValueConstraint>);
    as_boxed!(impl Clone for SomeType3<K: KeyConstraint, V: ValueConstraint>);

    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct ST3<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<K: ?Sized + KeyConstraint, V: ?Sized + KeyConstraint> SomeType3<K, V> for ST3<K, V> {
        fn say(&self, _k: K, _v: V) {}

        as_boxed!(impl SomeType3<K, V>);
        as_trait!(impl SomeType3<K, V>);
    }

    trait SomeType4<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>: Key {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType4<K, V>);
        as_trait!(SomeType4<K, V>);
    }

    as_boxed!(impl Hash for SomeType4<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>);
    as_boxed!(impl PartialEq for SomeType4<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>);
    as_boxed!(impl Eq for SomeType4<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>);
    as_boxed!(impl Clone for SomeType4<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>);

    trait SomeType5<K: 'static + KeyConstraint, V: 'static + ValueConstraint>: Key {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType5<K, V>);
        as_trait!(SomeType5<K, V>);
    }

    as_boxed!(impl Hash for SomeType5<K: 'static + KeyConstraint, V: 'static + ValueConstraint>);
    as_boxed!(impl PartialEq for SomeType5<K: 'static + KeyConstraint, V: 'static + ValueConstraint>);
    as_boxed!(impl Eq for SomeType5<K: 'static + KeyConstraint, V: 'static + ValueConstraint>);
    as_boxed!(impl Clone for SomeType5<K: 'static + KeyConstraint, V: 'static + ValueConstraint>);

    trait SomeType6<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>:
        Key
    {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType6<K, V>);
        as_trait!(SomeType6<K, V>);
    }

    as_boxed!(impl Hash for SomeType6<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>);
    as_boxed!(impl PartialEq for SomeType6<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>);
    as_boxed!(impl Eq for SomeType6<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>);
    as_boxed!(impl Clone for SomeType6<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>);

    trait SomeType7<
        K: 'static + ?Sized + KeyConstraint + Debug,
        V: 'static + ?Sized + ValueConstraint,
    >: Key
    {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType7<K, V>);
        as_trait!(SomeType7<K, V>);
    }

    as_boxed!(impl Hash for SomeType7<K: 'static + ?Sized + KeyConstraint + Debug, V: 'static + ?Sized + ValueConstraint>);
    as_boxed!(impl PartialEq for SomeType7<K: 'static + ?Sized + KeyConstraint + Debug, V: 'static + ?Sized + ValueConstraint>);
    as_boxed!(impl Eq for SomeType7<K: 'static + ?Sized + KeyConstraint + Debug, V: 'static + ?Sized + ValueConstraint>);
    as_boxed!(impl Clone for SomeType7<K: 'static + ?Sized + KeyConstraint + Debug, V: 'static + ?Sized + ValueConstraint>);
}
