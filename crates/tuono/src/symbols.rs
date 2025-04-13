use std::ops::Deref;

use syn::Ident;

pub struct Symbol(&'static str);

// The trait name that allow automatic struct/enum/type conversion
// to typescript
pub const TYPE_TRAIT: Symbol = Symbol("Type");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl Deref for Symbol {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
