use std::any::Any;
use std::sync::atomic::{AtomicI64, Ordering};

static CLONES: AtomicI64 = AtomicI64::new(0);
static ROOTS: AtomicI64 = AtomicI64::new(0);

pub struct Entry(i64);

impl Clone for Entry {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Ordering::SeqCst);
        Self(self.0)
    }
}

impl Entry {
    pub fn value(&self) -> i64 { self.0 }
}

pub struct Extensions {
    value: Box<dyn Any + Send + Sync>,
}

impl Extensions {
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    pub fn entry(&self) -> &Entry {
        self.value.downcast_ref().unwrap()
    }
}

pub struct Header {
    text: String,
}

impl Header {
    pub fn text(&self) -> &str { &self.text }
    pub fn length(&self) -> i64 { self.text.len() as i64 }
}

pub struct Request {
    extensions: Extensions,
    header: Header,
}

impl Request {
    pub fn new(value: i64) -> Self {
        ROOTS.fetch_add(1, Ordering::SeqCst);
        Self {
            extensions: Extensions { value: Box::new(Entry(value)) },
            header: Header { text: "forwarded".to_owned() },
        }
    }

    pub fn extensions(&self) -> &Extensions { &self.extensions }
    pub fn headers(&self) -> &Header { &self.header }
}

pub struct Canvas { total: i64 }

impl Canvas {
    pub fn draw_circle(&mut self, radius: i64) { self.total += radius; }
    pub fn total(&self) -> i64 { self.total }
}

pub struct Host { canvas: Canvas }

impl Host {
    pub fn new() -> Self { Self { canvas: Canvas { total: 0 } } }
    pub fn base_mut(&mut self) -> &mut Canvas { &mut self.canvas }
    pub fn total(&self) -> i64 { self.canvas.total }
}

pub fn clone_count() -> i64 { CLONES.load(Ordering::SeqCst) }
pub fn root_count() -> i64 { ROOTS.load(Ordering::SeqCst) }

pub fn consume(entry: &Entry) -> i64 { entry.value() }

pub struct View<'view> { entry: &'view Entry }

pub fn make_view(entry: &Entry) -> View<'_> { View { entry } }

pub fn render<F>(request: &Request, view: F) -> i64
where F: for<'view> Fn(&'view Request) -> View<'view> {
    view(request).entry.value()
}
