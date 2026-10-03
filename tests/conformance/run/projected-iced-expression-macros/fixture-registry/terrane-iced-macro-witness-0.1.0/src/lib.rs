use iced::{Element, advanced::widget::Tree};
use std::sync::atomic::{AtomicUsize, Ordering};
static CLONES: AtomicUsize = AtomicUsize::new(0);
pub struct State {
    pub label: String,
    pub children: Vec<State>,
}
impl Clone for State {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Ordering::SeqCst);
        Self {
            label: self.label.clone(),
            children: self.children.clone(),
        }
    }
}
impl State {
    pub fn new(label: String) -> Self {
        Self {
            label,
            children: Vec::new(),
        }
    }
    pub fn branch(label: String, children: Vec<State>) -> Self {
        Self { label, children }
    }
    pub fn get_label(&self) -> &str {
        &self.label
    }
}
pub fn inspect<F>(state: &State, render: F) -> String
where
    F: for<'a> Fn(&'a State) -> Element<'a, ()>,
{
    fn shape(tree: &Tree) -> (usize, usize, usize) {
        let mut nodes = 1;
        let mut branches = usize::from(!tree.children.is_empty());
        let mut depth = 0;
        for child in &tree.children {
            let (n, b, d) = shape(child);
            nodes += n;
            branches += b;
            depth = depth.max(d + 1);
        }
        (nodes, branches, depth)
    }
    let before = CLONES.load(Ordering::SeqCst);
    let element = render(state);
    let tree = Tree::new(element.as_widget());
    let (nodes, branches, depth) = shape(&tree);
    let clones = CLONES.load(Ordering::SeqCst) - before;
    format!("{nodes}:{branches}:{depth}:{clones}")
}
