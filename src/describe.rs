use http::Method;

#[derive(Debug)]
pub enum Description {
    And(Box<Description>, Box<Description>),
    AndThen(Box<Description>),
    Map(Box<Description>),
    Or(Box<Description>, Box<Description>),
    OrElse(Box<Description>),
    Empty,
    Fn(DescriptionFn),
    MapErr(Box<Description>),
    Recover(Box<Description>),
    Unify(Box<Description>),
    UntupleOne(Box<Description>),
    Any,
    Cors(Box<Description>),
    Log(Box<Description>),
}

#[derive(Debug, Clone, Copy)]
pub enum DescriptionFn {
    Method,
    MethodIs(&'static Method),
    Path(DescriptionPath),
    ContentType {
        type_: mime::Name<'static>,
        subtype: mime::Name<'static>,
    },
    Header {
        name: &'static str, // TODO: Add static parameter here
    },
    Header2, // TODO: Add static parameter here
    HeaderExact {
        name: &'static str,
        value: &'static str,
    },
    HeaderExactIgnoreCase {
        name: &'static str,
        value: &'static str,
    },
    HeaderOptional {
        name: &'static str, // TODO: Add static parameter here
    },
    HeaderOptional2, // TODO: Add static parameter here
    HeadersCloned,
    Body,
    Remote,
    CokieOptional {
        name: &'static str,
    },
    Query, // TODO: Add static parameter here
    QueryRaw,
    Extension, // TODO: Add static parameter here
}

#[derive(Debug, Clone, Copy)]
pub enum DescriptionPath {
    Tail,
    Full,
    Path(&'static str),
    End,
    Param, // TODO: Add static parameter here!
    Peek,
}
