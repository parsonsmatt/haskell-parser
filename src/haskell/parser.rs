/// Haskell parser.

use combine::*;

use haskell::syntax::*;

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

