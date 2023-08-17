// An enum representing different types of GrepItems
#[derive(PartialEq)]
pub(crate) enum GrepType {
    Impl,
    Fn,
    Struct,
    Enum,
    StructField,
    Const,
    Mod,
    Empty,
    Type,
    Trait,
}

// Prints the type of the item that's going to be grepped
impl std::fmt::Display for GrepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrepType::Impl => write!(f, "impl"),
            GrepType::Fn => write!(f, "fn"),
            GrepType::Struct => write!(f, "struct"),
            GrepType::Enum => write!(f, "enum"),
            GrepType::StructField => write!(f, "struct field"),
            GrepType::Const => write!(f, "const"),
            GrepType::Mod => write!(f, "mod"),
            GrepType::Empty => write!(f, ""),
            GrepType::Trait => write!(f, "trait"),
            GrepType::Type => write!(f, "type"),
        }
    }
}

// "Struct field" doesn't come from the public_api lib,
// so there's no need to convert from it.
// NOTE: Using From rather that FromStr, because FromStr requires "Err" implementation
impl<T> From<T> for GrepType
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let val_str = value.into().to_string();
        if val_str == "fn" {
            GrepType::Fn
        } else if val_str == "enum" {
            GrepType::Enum
        } else if val_str == "struct" {
            GrepType::Struct
        } else if val_str == "const" {
            GrepType::Const
        } else if val_str == "impl" {
            GrepType::Impl
        } else if val_str == "trait" {
            GrepType::Trait
        } else if val_str == "type" {
            GrepType::Type
        } else if val_str == "mod" {
            GrepType::Mod
        } else {
            GrepType::Empty
        }
    }
}

impl Default for GrepType {
    fn default() -> Self {
        GrepType::Empty
    }
}
