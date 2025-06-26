use std::fmt::{Debug, Display};

use super::any::*;

pub trait ToStringExtension {
    fn to_string(&self) -> String;
}

pub trait ToDebugString {
    fn to_debug_string(&self) -> String;
}

pub trait ToInstanceString {
    fn to_instance_string(&self) -> String;
}

impl<T: ?Sized> ToInstanceString for T {
    fn to_instance_string(&self) -> String {
        format!(
            "{{ type_name: {}, memory_address: {} }}",
            self.type_name(),
            self.memory_address()
        )
    }
}

impl<T: ?Sized + Debug> ToDebugString for T {
    fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: Display> ToStringExtension for Option<T> {
    fn to_string(&self) -> String {
        match self {
            Some(value) => format!("Some({})", value.to_string()),
            None => "None".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let mut o = Some(10);
        println!("{}", o.to_string());
        o = None;
        println!("{}", o.to_string());
    }

    #[test]
    fn to_debug_string() {
        let mut o = Some(10);
        println!("{}", o.to_debug_string());
        println!("{:?}", o);
        o = None;
        println!("{}", o.to_debug_string());
    }

    #[test]
    fn to_instance_string() {
        let o = Some(10);
        println!("{}", o.to_instance_string());
    }
}
