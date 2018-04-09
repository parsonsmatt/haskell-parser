/// A structure representing a Haskell module.
#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub extensions: Vec<Extension>,
    pub name: ModuleName,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

/// A language extension.
#[derive(Debug, PartialEq, Clone)]
pub enum Extension {
    OverloadedStrings,
}

/// An import.
#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub qualified: bool,
    pub module: ModuleName,
    pub alias: Option<ModuleName>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Qualified {
    Unqualified,
    Qualified,
    QualifiedAs(ModuleName)
}

/// A module name.
#[derive(Debug, PartialEq, Clone)]
pub struct ModuleName(pub String);

impl ModuleName {
    pub fn from_string(s: &str) -> Self {
        ModuleName(String::from(s)) 
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Data,
    Type,
    Newtype,
    Value,
    Splice,
}

/// An identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn from_string(s: &str) -> Self {
        Identifier(String::from(s)) 
    }
}
