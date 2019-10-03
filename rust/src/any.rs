use std::any::{ Any, type_name };

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
}

impl<T: ?Sized> AnyExtension for T {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}
