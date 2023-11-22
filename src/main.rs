#[derive(Debug)]
pub struct StrSplit<'haystack,'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack,'delimiter> StrSplit<'haystack,'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter:&'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack> Iterator for StrSplit<'haystack,'_> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delimiter) = remainder.find(self.delimiter) {
            let until_delimeter = &remainder[..next_delimiter];
            *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit At least give one result")
}

fn main() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);

    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);

    assert_eq!(until_char("hello world", 'o'), "hell");
}
