use std::any::{ Any, type_name };
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::string::*;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAnyMut for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait AnyExtension {
    fn type_name(&self) -> &'static str;
    fn address(&self) -> isize;
    fn hashcode(&self) -> u64;
    fn reference_equals(&self, other: &dyn Any) -> bool;
}

impl<T: ?Sized> AnyExtension for T {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn address(&self) -> isize {
        let s: String = format!("{:?}", self as *const T);
        match s.parse::<isize>() {
            Ok(v) => v,
            Err(_) => -1
        }
    }

    fn hashcode(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        hasher.write_isize(self.address());
        hasher.finish()
    }

    fn reference_equals(&self, other: &dyn Any) -> bool {
        let (p, p2) = (self.address(), other.address());
        p != -1 && p == p2
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_extension() {
        let a: Box<dyn AnyExtension> = Box::new(10);
        println!("type_name: {}, hash_code: {}, address: {:?}",
            a.as_ref().type_name(), a.as_ref().hashcode(), a.as_ref() as *const _);
        println!("type_name: {}, hash_code: {}, address: {:?}",
            a.type_name(), a.hashcode(), &a as *const _);

        let b: Box<dyn AnyExtension> = Box::new(10);
        println!("type_name: {}, hash_code: {}, address: {:?}",
            b.as_ref().type_name(), b.as_ref().hashcode(), b.as_ref() as *const _);
        println!("a == b: {}", a.as_ref().reference_equals(b.as_any()));
        assert!(!a.as_ref().reference_equals(b.as_any()));
    }
}