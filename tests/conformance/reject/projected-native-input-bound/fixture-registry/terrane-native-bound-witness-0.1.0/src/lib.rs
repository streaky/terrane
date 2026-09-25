#[derive(Clone, Copy)]
pub struct Radius {
    pub value: f32,
}

impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Clone, Copy)]
pub struct Border {
    pub radius: Radius,
}

pub fn make_border() -> Border {
    Border {
        radius: Radius { value: 0.0 },
    }
}
impl Border {
    pub fn rounded(self, radius: impl Into<Radius>) -> Self {
        Self {
            radius: radius.into(),
            ..self
        }
    }
}
