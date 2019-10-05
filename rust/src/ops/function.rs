
pub type Function<V, R> = Box<dyn Fn(V) -> R>;

pub type Action = Box<dyn Fn()>;

pub type Supplier<V> = Box<dyn Fn() -> V>;

pub type Consumer<V> = Box<dyn Fn(V)>;

pub type Predicate<V> = Box<dyn Fn(V) -> bool>;

pub type BiFunction<V, V2, R> = Box<dyn Fn(V, V2) -> R>;

pub type BiConsumer<V, V2> = Box<dyn Fn(V, V2)>;

pub type BiPredicate<V, V2> = Box<dyn Fn(V, V2) -> bool>;

pub type FunctionMut<V, R> = Box<dyn FnMut(V) -> R>;

pub type ActionMut = Box<dyn FnMut()>;

pub type SupplierMut<V> = Box<dyn FnMut() -> V>;

pub type ConsumerMut<V> = Box<dyn FnMut(V)>;

pub type PredicateMut<V> = Box<dyn FnMut(V) -> bool>;

pub type BiFunctionMut<V, V2, R> = Box<dyn FnMut(V, V2) -> R>;

pub type BiConsumerMut<V, V2> = Box<dyn FnMut(V, V2)>;

pub type BiPredicateMut<V, V2> = Box<dyn FnMut(V, V2) -> bool>;

pub type FunctionOnce<V, R> = Box<dyn FnOnce(V) -> R>;

pub type ActionOnce = Box<dyn FnOnce()>;

pub type SupplierOnce<V> = Box<dyn FnOnce() -> V>;

pub type ConsumerOnce<V> = Box<dyn FnOnce(V)>;

pub type PredicateOnce<V> = Box<dyn FnOnce(V) -> bool>;

pub type BiFunctionOnce<V, V2, R> = Box<dyn FnOnce(V, V2) -> R>;

pub type BiConsumerOnce<V, V2> = Box<dyn FnOnce(V, V2)>;

pub type BiPredicateOnce<V, V2> = Box<dyn FnOnce(V, V2) -> bool>;

pub type FunctionRef<V, R> = Box<dyn Fn(&V) -> R>;

pub type ConsumerRef<V> = Box<dyn Fn(&V)>;

pub type PredicateRef<V> = Box<dyn Fn(&V) -> bool>;

pub type BiFunctionRef<V, V2, R> = Box<dyn Fn(&V, &V2) -> R>;

pub type BiConsumerRef<V, V2> = Box<dyn Fn(&V, &V2)>;

pub type BiPredicateRef<V, V2> = Box<dyn Fn(&V, &V2) -> bool>;

pub type FunctionRefMut<V, R> = Box<dyn FnMut(&V) -> R>;

pub type ConsumerRefMut<V> = Box<dyn FnMut(&V)>;

pub type PredicateRefMut<V> = Box<dyn FnMut(&V) -> bool>;

pub type BiFunctionRefMut<V, V2, R> = Box<dyn FnMut(&V, &V2) -> R>;

pub type BiConsumerRefMut<V, V2> = Box<dyn FnMut(&V, &V2)>;

pub type BiPredicateRefMut<V, V2> = Box<dyn FnMut(&V, &V2) -> bool>;

pub type FunctionRefOnce<V, R> = Box<dyn FnOnce(&V) -> R>;

pub type ConsumerRefOnce<V> = Box<dyn FnOnce(&V)>;

pub type PredicateRefOnce<V> = Box<dyn FnOnce(&V) -> bool>;

pub type BiFunctionRefOnce<V, V2, R> = Box<dyn FnOnce(&V, &V2) -> R>;

pub type BiConsumerRefOnce<V, V2> = Box<dyn FnOnce(&V, &V2)>;

pub type BiPredicateRefOnce<V, V2> = Box<dyn FnOnce(&V, &V2) -> bool>;
