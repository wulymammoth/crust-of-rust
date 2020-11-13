#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// NOTE: anonymous lifetimes
// - where we tell the compiler to guess the lifetime and only works when
// - there is one possible guess; type-inference for lifetimes (elision)
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
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
        // NOTE: if `ref` isn't used, then remainder's value is moved
        // - we want a mutable reference to the value inside remainder because we want to modify
        // the existing value
        // - take a reference to as opposed to `Some(&mut remainder)`
        // - we need `as_mut()` here because we want to access self (remainder) not a new pointer
        // to the Option<T> of a copy of the remainder (otherwise it hangs)
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delimiter) = remainder.find(self.delimiter) {
            let head = &remainder[..next_delimiter];
            // NOTE: why do we need the deref operator here?
            // - not of the same type
            // - LHS type: &mut &'a str
            // - RHS type: &'a str
            // * want to assign into where `remainder` is pointing
            *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
            Some(head)
        } else {
            self.remainder.take() // "takes" the value of the option leaving `None` in its place
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    // NOTE: is the element empty or an element we haven't yet yielded?
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
