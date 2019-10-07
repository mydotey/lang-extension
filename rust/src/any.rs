use std::any::{ Any, type_name };
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
    fn memory_address(&self) -> usize;
    fn to_instance_string(&self) -> String;
    fn reference_equals(&self, other: &dyn Any) -> bool;
}

impl<T: ?Sized> AnyExtension for T {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn memory_address(&self) -> usize {
        self as *const T as *const () as usize
    }

    fn to_instance_string(&self) -> String {
        format!("{{ type_name: {}, memory_address: {} }}", self.type_name(), self.memory_address())
    }

    fn reference_equals(&self, other: &dyn Any) -> bool {
        self.memory_address() == other.memory_address()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_extension() {
        let a: Box<dyn Any> = Box::new(10);
        println!("instance_string: {}, type_name: {}, memory_address: {:?}",
            a.as_ref().to_instance_string(), a.as_ref().type_name(), a.as_ref().memory_address());

        let b: Box<dyn Any> = Box::new(10);
        println!("instance_string: {}, type_name: {}, memory_address: {:?}",
            a.as_ref().to_instance_string(), b.as_ref().type_name(), b.as_ref().memory_address());

        println!("a == b: {}", a.as_ref().reference_equals(b.as_any()));
        assert!(!a.as_ref().reference_equals(b.as_any()));
    }
}