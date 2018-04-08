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

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Data,
    Type,
    Newtype,
    Value,
    Splice,
}
