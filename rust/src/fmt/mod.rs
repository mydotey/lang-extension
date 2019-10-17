use std::fmt::{ Display, Debug };

use super::any::*;

pub trait ToStringExtension<T: Display> {
    fn to_string(&self) -> String;
}

pub trait ToDebugString<T: Debug> {
    fn to_debug_string(&self) -> String;
}

pub trait ToInstanceString {
    fn to_instance_string(&self) -> String;
}

impl<T: ?Sized> ToInstanceString for T {
    fn to_instance_string(&self) -> String {
        format!("{{ type_name: {}, memory_address: {} }}", self.type_name(), self.memory_address())
    }
}

impl<T: Display> ToStringExtension<T> for Option<T> {
    fn to_string(&self) -> String {
        match self {
            Some(value) => format!("Some({})", value.to_string()),
            None => "None".to_string()
        }
    }
}

impl<T: Debug> ToDebugString<T> for Option<T> {
    fn to_debug_string(&self) -> String {
        match self {
            Some(value) => format!("Some({:?})", value),
            None => "None".to_string()
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
        o = None;
        println!("{}", o.to_debug_string());
    }

    #[test]
    fn to_instance_string() {
        let o = Some(10);
        println!("{}", o.to_instance_string());
    }
}
