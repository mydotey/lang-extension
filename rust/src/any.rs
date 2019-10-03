use std::any::{ Any, type_name, TypeId };

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

pub trait RawTypeName {
    fn raw_type_name(&self) -> &'static str;
}

impl<T: ?Sized> RawTypeName for T {
    fn raw_type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

pub trait RawTypeId {
    fn raw_type_id(&self) -> TypeId;
}

impl<T: 'static + ?Sized> RawTypeId for T {
    fn raw_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
