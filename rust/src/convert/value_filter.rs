use std::any::type_name;
use std::fmt;
use std::sync::Arc;

use crate::any::*;
use crate::ops::function::*;
use crate::*;

pub trait RawValueFilter: Value + Sync + Send {
    fn filter_raw(&self, value: Box<dyn Value>) -> Option<Box<dyn Value>>;

    as_trait!(RawValueFilter);
    as_boxed!(RawValueFilter);
}

boxed_value_trait!(RawValueFilter);

pub trait ValueFilter<V: ?Sized + ValueConstraint>: RawValueFilter {
    fn filter(&self, value: Box<V>) -> Option<Box<V>>;

    as_boxed!(ValueFilter<V>);
}

boxed_value_trait!(ValueFilter<V: ?Sized + ValueConstraint>);

#[derive(Clone)]
pub struct DefaultValueFilter<V: ?Sized + ValueConstraint> {
    filter: Function<Box<V>, Option<Box<V>>>,
}

impl<V: ?Sized + ValueConstraint> DefaultValueFilter<V> {
    pub fn new(filter: Box<dyn Fn(Box<V>) -> Option<Box<V>>>) -> Self {
        Self::wrap(Arc::new(filter))
    }

    pub fn wrap(filter: Arc<Box<dyn Fn(Box<V>) -> Option<Box<V>>>>) -> Self {
        Self { filter: filter }
    }
}

impl<V: ?Sized + ValueConstraint> ValueFilter<V> for DefaultValueFilter<V> {
    fn filter(&self, value: Box<V>) -> Option<Box<V>> {
        self.filter.as_ref()(value)
    }

    as_boxed!(impl ValueFilter<V>);
}

impl<V: ?Sized + ValueConstraint> PartialEq for DefaultValueFilter<V> {
    fn eq(&self, other: &Self) -> bool {
        self.filter.as_ref().reference_equals(other.filter.as_ref())
    }
}

impl<V: ?Sized + ValueConstraint> Eq for DefaultValueFilter<V> {}

impl<V: ?Sized + ValueConstraint> fmt::Debug for DefaultValueFilter<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {{ filter: {} }}",
            type_name::<DefaultValueFilter<V>>(),
            self.filter.type_name()
        )
    }
}

unsafe impl<V: ?Sized + ValueConstraint> Sync for DefaultValueFilter<V> {}
unsafe impl<V: ?Sized + ValueConstraint> Send for DefaultValueFilter<V> {}

impl<V: ?Sized + ValueConstraint> RawValueFilter for DefaultValueFilter<V> {
    fn filter_raw(&self, value: Box<dyn Value>) -> Option<Box<dyn Value>> {
        match value.as_ref().as_any_ref().downcast_ref::<V>() {
            Some(value) => self.filter.as_ref()(Box::new(value.clone()))
                .map(|v| Value::clone_boxed(v.as_ref())),
            None => None,
        }
    }

    as_trait!(impl RawValueFilter);
    as_boxed!(impl RawValueFilter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_filter_test() {
        let filter =
            DefaultValueFilter::<i32>::new(Box::new(|v| if *v > 10 { Some(v) } else { None }));
        let v = filter.filter(Box::new(10));
        assert_eq!(None, v);
        let v = filter.filter(Box::new(11));
        assert_eq!(Box::new(11), v.unwrap());
        let v = RawValueFilter::filter_raw(&filter, Box::new(11));
        let expected: Box<dyn Value> = Box::new(11);
        assert_eq!(&expected, &v.unwrap());
    }
}
