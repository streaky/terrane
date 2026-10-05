pub struct CopyOnly<T: Copy>(T);
pub struct Outer<T>(pub T);

pub fn nested() -> Outer<CopyOnly<bool>> { Outer(CopyOnly(true)) }
pub fn read(value: &Outer<CopyOnly<bool>>) -> bool { value.0.0 }
