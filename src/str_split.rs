#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

// NOTE: anonymous lifetimes
// - where we tell the compiler to guess the lifetime and only works when
// - there is one possible guess; type-inference for lifetimes (elision)
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// why is the lifetime specifier needed next to `impl`?
// - makes the impl generic over a lifetime
// - it's a specifier for the implementation and not the type (like generics)
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter) = self.remainder.find(self.delimiter) {
            let head = &self.remainder[..next_delimiter];
            self.remainder = &self.remainder[(next_delimiter + self.delimiter.len())..];
            Some(head)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let tail = self.remainder;
            // NOTE: why is assigning a string literal to remainder okay here?
            // - `remainder` is &'a str
            // - "" is &'static str
            // * can assign to `remainder` as long as what's being assigned has these invariants
            //   1. same type
            //   2. greater than or equal lifetime
            self.remainder = "";
            Some(tail)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}
