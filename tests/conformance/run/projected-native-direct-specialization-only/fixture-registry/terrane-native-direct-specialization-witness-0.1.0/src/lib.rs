pub struct Cell<T>(pub T);

pub fn make<T: Default>() -> Cell<T> {
    Cell(T::default())
}

pub fn consume<T: std::fmt::Display>(cell: Cell<T>) -> String {
    cell.0.to_string()
}
