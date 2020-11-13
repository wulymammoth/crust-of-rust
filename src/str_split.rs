#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D, // NOTE: if we use a String here instead, it is expensive and requires an allocator (cannot go embedded as it may not have an allocator)
}

// NOTE: anonymous lifetimes
// - where we tell the compiler to guess the lifetime and only works when
// - there is one possible guess; type-inference for lifetimes (elision)
#[allow(dead_code)]
impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// why is the lifetime specifier needed next to `impl`?
// - makes the impl generic over a lifetime
// - it's a specifier for the implementation and not the type (like generics)
// - anon lifetime elision to match the `StrSplit` args, but we don't care about that second
// lifetime
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    // we can use the anon lifetime here, because it isn't needed
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: if `ref` isn't used, then remainder's value is moved
        // - we want a mutable reference to the value inside remainder because we want to modify
        // the existing value
        // - take a reference to as opposed to `Some(&mut remainder)`
        // - we need `as_mut()` here because we want to access self (remainder) not a new pointer
        // to the Option<T> of a copy of the remainder (otherwise it hangs)
        let remainder = self.remainder.as_mut()?;
        if let Some((next_delimiter_start, _next_delimiter_end)) =
            self.delimiter.find_next(remainder)
        {
            let head = &remainder[..next_delimiter_start];
            // NOTE: why do we need the deref operator here?
            // - not of the same type
            // - LHS type: &mut &'haystack str
            // - RHS type: &'haystack str
            // * want to assign into where `remainder` is pointing
            *remainder = &remainder[_next_delimiter_end..];
            Some(head)
        } else {
            self.remainder.take() // "takes" the value of the option leaving `None` in its place
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + 1))
    }
}

#[allow(dead_code)]
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
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

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}
