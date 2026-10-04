use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Token {
    Good,
}

impl FromStr for Token {
    type Err = std::io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "good" => Ok(Self::Good),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "bad token")),
        }
    }
}

pub trait Contains<T> {
    fn contains(&self, value: T) -> bool;
}

impl<'a> Contains<&'a Token> for Token {
    fn contains(&self, _value: &'a Token) -> bool {
        true
    }
}

impl Token {
    pub fn contains<T>(&self, value: T) -> bool
    where
        Self: Contains<T>,
    {
        <Self as Contains<T>>::contains(self, value)
    }
}

#[derive(Clone)]
pub struct Basket {
    pub items: Vec<Token>,
}

impl Basket {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn len(&self) -> i64 {
        self.items.len() as i64
    }
}

#[derive(Clone, Copy)]
pub struct Response(pub i64);

impl Response {
    pub fn value(&self) -> i64 {
        self.0
    }
}

pub trait Factory {
    type Output;
    fn create() -> Self::Output;
}

impl Factory for Token {
    type Output = Response;

    fn create() -> Self::Output {
        Response(7)
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Token {
    fn into_response(self) -> Response {
        Response(11)
    }
}

#[derive(Clone, Copy)]
pub struct Element(pub i64);

impl Element {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<Token> for Element {
    fn from(_value: Token) -> Self {
        Self(13)
    }
}

pub fn destination<T: Default>() -> T {
    T::default()
}

#[derive(Default)]
pub struct Selected(pub i64);

impl Selected {
    pub fn value(&self) -> i64 {
        self.0 + 17
    }
}
