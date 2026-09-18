pub struct Probe {
    label: String,
}

impl Drop for Probe {
    fn drop(&mut self) {
        println!("drop {}", self.label);
    }
}

pub fn probe(label: String) -> Probe {
    println!("make {label}");
    Probe { label }
}
