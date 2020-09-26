use super::super::utils::extract_pkgname_prefix;
use super::unreasoned::UnreasonedDependency;
use pipe_trait::*;

#[derive(Debug, Copy, Clone)]
pub struct ReasonedDependency<Name, Range, Reason>
where
    Name: AsRef<str>,
    Range: AsRef<str>,
    Reason: AsRef<str>,
{
    pub name: Name,
    pub range: Range,
    pub reason: Option<Reason>,
}

impl<Name, Range, Reason> ReasonedDependency<Name, Range, Reason>
where
    Name: AsRef<str>,
    Range: AsRef<str>,
    Reason: AsRef<str>,
{
    pub fn into_unreasoned_dependency(self) -> UnreasonedDependency<Name, Range> {
        UnreasonedDependency {
            name: self.name,
            range: self.range,
        }
    }

    pub fn as_str(&self) -> ReasonedDependency<&str, &str, &str> {
        ReasonedDependency {
            name: self.name(),
            range: self.range(),
            reason: self.reason(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn range(&self) -> &str {
        self.range.as_ref()
    }

    pub fn reason(&self) -> Option<&str> {
        if let Some(reason) = &self.reason {
            Some(reason.as_ref())
        } else {
            None
        }
    }
}

impl<'a> ReasonedDependency<&'a str, &'a str, &'a str> {
    pub fn new(text: &'a str) -> Self {
        let mut parts = text.splitn(1, ':');
        let (name, range) = parts.next().unwrap().pipe(extract_pkgname_prefix);
        let reason = parts.next().map(|x| x.trim());
        ReasonedDependency {
            name,
            range,
            reason,
        }
    }
}
