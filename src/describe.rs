#[derive(Debug)]
pub enum Description {
    And(Box<Description>, Box<Description>),
    AndThen(Box<Description>),
    Map(Box<Description>),
    Or(Box<Description>, Box<Description>),
    OrElse(Box<Description>),
    Empty,
    Fn,
    MapErr(Box<Description>),
    Recover(Box<Description>),
    Unify(Box<Description>),
    UntupleOne(Box<Description>),
    Any,
    Cors(Box<Description>),
    Log(Box<Description>),
}
