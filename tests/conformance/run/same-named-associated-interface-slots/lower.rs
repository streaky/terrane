// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: same-named-associated-interface-slots
pub trait NumberViewProtocol<TerraneType726573706F6E7365>: Send + Sync {
    fn clone_box(&self) -> Box<dyn NumberViewProtocol<TerraneType726573706F6E7365>>;
    fn separate_box(&self) -> Box<dyn NumberViewProtocol<TerraneType726573706F6E7365>>;
    fn render(&self) -> TerraneType726573706F6E7365;
}
impl<TerraneType726573706F6E7365> Clone
for Box<dyn NumberViewProtocol<TerraneType726573706F6E7365>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct NumberView<TerraneType726573706F6E7365>(
    Box<dyn NumberViewProtocol<TerraneType726573706F6E7365>>,
);
impl<TerraneType726573706F6E7365> Clone for NumberView<TerraneType726573706F6E7365> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType726573706F6E7365> NumberView<TerraneType726573706F6E7365> {
    pub fn render(&self) -> TerraneType726573706F6E7365 {
        self.0.render()
    }
}
pub trait TextViewProtocol<TerraneType726573706F6E7365>: Send + Sync {
    fn clone_box(&self) -> Box<dyn TextViewProtocol<TerraneType726573706F6E7365>>;
    fn separate_box(&self) -> Box<dyn TextViewProtocol<TerraneType726573706F6E7365>>;
    fn render(&self) -> TerraneType726573706F6E7365;
}
impl<TerraneType726573706F6E7365> Clone
for Box<dyn TextViewProtocol<TerraneType726573706F6E7365>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TextView<TerraneType726573706F6E7365>(
    Box<dyn TextViewProtocol<TerraneType726573706F6E7365>>,
);
impl<TerraneType726573706F6E7365> Clone for TextView<TerraneType726573706F6E7365> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType726573706F6E7365> TextView<TerraneType726573706F6E7365> {
    pub fn render(&self) -> TerraneType726573706F6E7365 {
        self.0.render()
    }
}
#[derive(Clone)]
pub struct CombinedView {}
impl CombinedView {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(&self) -> String {
        return String::from("shared");
    }
}
impl NumberViewProtocol<String> for CombinedView {
    fn clone_box(&self) -> Box<dyn NumberViewProtocol<String>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn NumberViewProtocol<String>> {
        Box::new(self.clone())
    }
    fn render(&self) -> String {
        CombinedView::render(&*self)
    }
}
impl From<CombinedView> for NumberView<String> {
    fn from(value: CombinedView) -> Self {
        Self(Box::new(value))
    }
}
impl TextViewProtocol<String> for CombinedView {
    fn clone_box(&self) -> Box<dyn TextViewProtocol<String>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn TextViewProtocol<String>> {
        Box::new(self.clone())
    }
    fn render(&self) -> String {
        CombinedView::render(&*self)
    }
}
impl From<CombinedView> for TextView<String> {
    fn from(value: CombinedView) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let numeric: NumberView<String>;
    let textual: TextView<String>;
    numeric = <NumberView<String>>::from(CombinedView::terrane_construct());
    textual = <TextView<String>>::from(CombinedView::terrane_construct());
    println!("{}", terrane_scalar_support::scalar_text(&numeric.render()));
    println!("{}", terrane_scalar_support::scalar_text(&textual.render()));
}
