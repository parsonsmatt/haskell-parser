/// Haskell parser.

use combine::*;
use combine::parser::char::*;
use syntax::*;

parser! {
    fn module[I]()(I) -> Module
        where [I: Stream<Item = char>]
    {
        struct_parser!{
            Module { 
                extensions: value(vec![]),
                name: many1(any()).map(|s:String| ModuleName(s)),
                imports: value(vec![]),
                declarations: value(vec![])
            }
        }
    }
}

parser! {
    /// Parse a Haskell module name. This is a string matching the regex:
    /// [A-Z][_a-zA-Z\.]
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate combine;
    /// # extern crate haskell_parser;
    /// # fn main() {
    /// use haskell_parser::parser::module_name;
    /// use haskell_parser::syntax::ModuleName;
    /// use combine::Parser;
    ///
    /// let x = module_name().parse("Control.Monad");
    /// assert_eq!(
    ///     Ok(
    ///         (ModuleName::from_string("Control.Monad"), "")
    ///     ), 
    ///     x);
    /// # }
    /// ```
    pub fn module_name[I]()(I) -> ModuleName
        where [I: Stream<Item = char>]
    {
        (upper(), many(alpha_num().or(char('.')).or(char('_'))))
            .map(|(c, cs): (char, String)| 
                 ModuleName(format!("{}{}", c, cs))
            ).skip(spaces())
    }
}

parser! {
    /// Parse a Haskell identifier. This is a string matching the regex:
    /// [_a-z][_a-zA-Z']
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate combine;
    /// # extern crate haskell_parser;
    /// # fn main() {
    /// use haskell_parser::parser::ident;
    /// use haskell_parser::syntax::Identifier;
    /// use combine::Parser;
    ///
    /// let x = ident().parse("valid");
    /// assert_eq!(
    ///     Ok(
    ///         (Identifier(String::from("valid")), "")
    ///     ), 
    ///     x);
    /// # }
    /// ```
    pub fn ident[I]()(I) -> Identifier
        where [I: Stream<Item = char>]
    {
        let first_ident_char = lower().or(char('_'));
        let latter_char = alpha_num().or(one_of(vec!['\'', '_']));
        ( first_ident_char,
          many(latter_char),
          spaces()
        ).map(|(c, cs, _) : (char, String, _)| 
              Identifier(format!("{}{}", c, cs))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn is_err<T, E>(x : Result<T, E>) -> bool {
        match x {
            Ok(_) => false,
            _ => true,
        }
    }

    fn parse_as<A, E>(x: Result<(A, &str), E>, t: A) 
        where A: Debug + PartialEq, E: Debug + PartialEq
    {
        assert_eq!(x, Ok((t, ""))) 
    }

    // identifier tests
    mod idents {
        use super::*;
        #[test]
        fn identifiers() {
            parse_as(
                ident().parse("hello"),
                Identifier::from_string("hello")
            ) 
        }

        #[test]
        fn underscore() {
            parse_as(
                ident().parse("_Left"),
                Identifier::from_string("_Left")
            ) 
        }

        #[test]
        fn primes() {
            parse_as(
                ident().parse("don't"),
                Identifier::from_string("don't")
            )
        }

        #[test]
        fn digits() {
            parse_as(
                ident().parse("foo22"),
                Identifier::from_string("foo22")
            ) 
        }

        #[test]
        fn must_be_lowercase_first() {
            assert!(is_err(ident().parse("Left")));
            assert!(is_err(ident().parse("1asdf")));
            assert!(is_err(ident().parse("'foobar")))
        }
    }

    mod module_names {
        use super::*;    

        #[test]
        fn module_name_passes() {
            parse_as(
                module_name().parse("Foobar"),
                ModuleName::from_string("Foobar")
            ) 
        }

        #[test]
        fn can_have_dots() {
            parse_as(
                module_name().parse("Control.Monad"),
                ModuleName::from_string("Control.Monad")
            ) 
        }

        #[test]
        fn can_have_underscores() {
            parse_as(
                module_name().parse("Control_Monad"),
                ModuleName::from_string("Control_Monad")
            ) 
        }

        #[test]
        fn invalid_modules() {
            let parse = |x| module_name().parse(x);
            assert!(is_err(parse("foo")));
            assert!(is_err(parse("_asdf")));
            assert!(is_err(parse("3werty")));
            assert!(is_err(parse("Hello'World")))
        }
    }
}
