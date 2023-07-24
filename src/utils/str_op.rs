pub trait StrPlus {
    fn plus(&self, s: &str) -> String;
}

impl StrPlus for str {
    fn plus(&self, s: &str) -> String {
        format!("{self} {s}")
    }
}

pub trait StrOps {
    /// Captilises the first letter in s.
    fn capitalise(&self) -> String;
}

impl StrOps for String {
    /// Captilises the first letter in s.
    fn capitalise(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

pub trait VecStrOps {
    fn flat_concat(self, join: &'static str) -> Option<String>;
}

impl VecStrOps for Vec<Option<String>> {
    fn flat_concat(self, join: &'static str) -> Option<String> {
        self.into_iter().flatten().reduce(|mut acc, e| {
            acc.push_str(join);
            acc.push_str(&e);
            acc
        })
    }
}
