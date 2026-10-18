use std::{env, fs, path::PathBuf};

fn main() {
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(
        output.join("api.rs"),
        r#"
pub mod builtin {
    #[derive(Clone, Copy)]
    pub struct Vector2 {
        x: f32,
        y: f32,
    }

    impl Vector2 {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        pub fn length(&self) -> f32 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
}

pub mod classes {
    use super::builtin::Vector2;

    pub struct Node2D {
        position: Vector2,
    }

    impl Node2D {
        pub fn new(x: f32, y: f32) -> Self {
            Self { position: Vector2::new(x, y) }
        }

        pub fn get_position(&self) -> Vector2 {
            self.position
        }
    }
}
"#,
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
