//! Cookie Filters

use std::error::Error as StdError;

use headers::Cookie;

use super::header;
use describe::DescriptionFn;
use filter::{filter_fn_one, Filter, One};
use never::Never;
use reject::Rejection;

/// Creates a `Filter` that requires a cookie by name.
///
/// If found, extracts the value of the cookie, otherwise rejects.
pub fn cookie(name: &'static str) -> impl Filter<Extract = One<String>, Error = Rejection> + Copy {
    header::header2().and_then(move |cookie: Cookie| {
        cookie
            .get(name)
            .map(String::from)
            .ok_or_else(|| ::reject::known(MissingCookie(name)))
    })
}

/// Creates a `Filter` that looks for an optional cookie by name.
///
/// If found, extracts the value of the cookie, otherwise continues
/// the request, extracting `None`.
pub fn optional(
    name: &'static str,
) -> impl Filter<Extract = One<Option<String>>, Error = Never> + Copy {
    header::optional2()
        .map(move |opt: Option<Cookie>| opt.and_then(|cookie| cookie.get(name).map(String::from)))
}

#[doc(hidden)]
#[deprecated(note = "optional filters will be generalized")]
pub fn optional_value<U, F>(
    name: &'static str,
    func: F,
) -> impl Filter<Extract = One<Option<U>>, Error = Never> + Copy
where
    F: Fn(&str) -> U + Copy,
    U: Send,
{
    use headers::HeaderMapExt;
    filter_fn_one(DescriptionFn::CokieOptional { name }, move |route| {
        Ok(route
            .headers()
            .typed_get()
            .and_then(|cookie: Cookie| cookie.get(name).map(func)))
    })
}

// ===== Rejections =====

#[derive(Debug)]
pub(crate) struct MissingCookie(&'static str);

impl ::std::fmt::Display for MissingCookie {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Missing request cookie '{}'", self.0)
    }
}

impl StdError for MissingCookie {
    fn description(&self) -> &str {
        "Missing request cookie"
    }
}
