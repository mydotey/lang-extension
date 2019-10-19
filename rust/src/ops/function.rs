use std::sync::Arc;

pub type Function<V, R> = Arc<Box<dyn Fn(V) -> R>>;

pub type Action = Arc<Box<dyn Fn()>>;

pub type Supplier<V> = Arc<Box<dyn Fn() -> V>>;

pub type Consumer<V> = Arc<Box<dyn Fn(V)>>;

pub type Predicate<V> = Arc<Box<dyn Fn(V) -> bool>>;

pub type BiFunction<V, V2, R> = Arc<Box<dyn Fn(V, V2) -> R>>;

pub type BiConsumer<V, V2> = Arc<Box<dyn Fn(V, V2)>>;

pub type BiPredicate<V, V2> = Arc<Box<dyn Fn(V, V2) -> bool>>;

pub type FunctionMut<V, R> = Arc<Box<dyn FnMut(V) -> R>>;

pub type ActionMut = Arc<Box<dyn FnMut()>>;

pub type SupplierMut<V> = Arc<Box<dyn FnMut() -> V>>;

pub type ConsumerMut<V> = Arc<Box<dyn FnMut(V)>>;

pub type PredicateMut<V> = Arc<Box<dyn FnMut(V) -> bool>>;

pub type BiFunctionMut<V, V2, R> = Arc<Box<dyn FnMut(V, V2) -> R>>;

pub type BiConsumerMut<V, V2> = Arc<Box<dyn FnMut(V, V2)>>;

pub type BiPredicateMut<V, V2> = Arc<Box<dyn FnMut(V, V2) -> bool>>;

pub type FunctionOnce<V, R> = Arc<Box<dyn FnOnce(V) -> R>>;

pub type ActionOnce = Arc<Box<dyn FnOnce()>>;

pub type SupplierOnce<V> = Arc<Box<dyn FnOnce() -> V>>;

pub type ConsumerOnce<V> = Arc<Box<dyn FnOnce(V)>>;

pub type PredicateOnce<V> = Arc<Box<dyn FnOnce(V) -> bool>>;

pub type BiFunctionOnce<V, V2, R> = Arc<Box<dyn FnOnce(V, V2) -> R>>;

pub type BiConsumerOnce<V, V2> = Arc<Box<dyn FnOnce(V, V2)>>;

pub type BiPredicateOnce<V, V2> = Arc<Box<dyn FnOnce(V, V2) -> bool>>;

pub type FunctionRef<V, R> = Arc<Box<dyn Fn(&V) -> R>>;

pub type ConsumerRef<V> = Arc<Box<dyn Fn(&V)>>;

pub type PredicateRef<V> = Arc<Box<dyn Fn(&V) -> bool>>;

pub type BiFunctionRef<V, V2, R> = Arc<Box<dyn Fn(&V, &V2) -> R>>;

pub type BiConsumerRef<V, V2> = Arc<Box<dyn Fn(&V, &V2)>>;

pub type BiPredicateRef<V, V2> = Arc<Box<dyn Fn(&V, &V2) -> bool>>;

pub type FunctionRefMut<V, R> = Arc<Box<dyn FnMut(&V) -> R>>;

pub type ConsumerRefMut<V> = Arc<Box<dyn FnMut(&V)>>;

pub type PredicateRefMut<V> = Arc<Box<dyn FnMut(&V) -> bool>>;

pub type BiFunctionRefMut<V, V2, R> = Arc<Box<dyn FnMut(&V, &V2) -> R>>;

pub type BiConsumerRefMut<V, V2> = Arc<Box<dyn FnMut(&V, &V2)>>;

pub type BiPredicateRefMut<V, V2> = Arc<Box<dyn FnMut(&V, &V2) -> bool>>;

pub type FunctionRefOnce<V, R> = Arc<Box<dyn FnOnce(&V) -> R>>;

pub type ConsumerRefOnce<V> = Arc<Box<dyn FnOnce(&V)>>;

pub type PredicateRefOnce<V> = Arc<Box<dyn FnOnce(&V) -> bool>>;

pub type BiFunctionRefOnce<V, V2, R> = Arc<Box<dyn FnOnce(&V, &V2) -> R>>;

pub type BiConsumerRefOnce<V, V2> = Arc<Box<dyn FnOnce(&V, &V2)>>;

pub type BiPredicateRefOnce<V, V2> = Arc<Box<dyn FnOnce(&V, &V2) -> bool>>;
