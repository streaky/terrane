// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: deep-inheritance-field-forwarding
#[derive(Clone)]
pub struct Level0Storage {
    pub f0: terrane_int_support::Int,
}
impl Level0Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level0 {
    Own(Level0Storage),
    Level1(Level1),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level0 {
    pub fn terrane_construct() -> Self { Self::Own(Level0Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level1(value) => value.terrane_field_f0(),
            Self::Level2(value) => value.terrane_field_f0(),
            Self::Level3(value) => value.terrane_field_f0(),
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level1(value) => value.terrane_field_f0_mut(),
            Self::Level2(value) => value.terrane_field_f0_mut(),
            Self::Level3(value) => value.terrane_field_f0_mut(),
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
}
#[derive(Clone)]
pub struct Level1Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
}
impl Level1Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level1 {
    Own(Level1Storage),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level1 {
    pub fn terrane_construct() -> Self { Self::Own(Level1Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level2(value) => value.terrane_field_f0(),
            Self::Level3(value) => value.terrane_field_f0(),
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level2(value) => value.terrane_field_f0_mut(),
            Self::Level3(value) => value.terrane_field_f0_mut(),
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level2(value) => value.terrane_field_f1(),
            Self::Level3(value) => value.terrane_field_f1(),
            Self::Level4(value) => value.terrane_field_f1(),
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level2(value) => value.terrane_field_f1_mut(),
            Self::Level3(value) => value.terrane_field_f1_mut(),
            Self::Level4(value) => value.terrane_field_f1_mut(),
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
}
#[derive(Clone)]
pub struct Level2Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
}
impl Level2Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level2 {
    Own(Level2Storage),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level2 {
    pub fn terrane_construct() -> Self { Self::Own(Level2Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level3(value) => value.terrane_field_f0(),
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level3(value) => value.terrane_field_f0_mut(),
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level3(value) => value.terrane_field_f1(),
            Self::Level4(value) => value.terrane_field_f1(),
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level3(value) => value.terrane_field_f1_mut(),
            Self::Level4(value) => value.terrane_field_f1_mut(),
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level3(value) => value.terrane_field_f2(),
            Self::Level4(value) => value.terrane_field_f2(),
            Self::Level5(value) => value.terrane_field_f2(),
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level3(value) => value.terrane_field_f2_mut(),
            Self::Level4(value) => value.terrane_field_f2_mut(),
            Self::Level5(value) => value.terrane_field_f2_mut(),
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
}
#[derive(Clone)]
pub struct Level3Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
}
impl Level3Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level3 {
    Own(Level3Storage),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level3 {
    pub fn terrane_construct() -> Self { Self::Own(Level3Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level4(value) => value.terrane_field_f1(),
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level4(value) => value.terrane_field_f1_mut(),
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level4(value) => value.terrane_field_f2(),
            Self::Level5(value) => value.terrane_field_f2(),
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level4(value) => value.terrane_field_f2_mut(),
            Self::Level5(value) => value.terrane_field_f2_mut(),
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level4(value) => value.terrane_field_f3(),
            Self::Level5(value) => value.terrane_field_f3(),
            Self::Level6(value) => value.terrane_field_f3(),
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level4(value) => value.terrane_field_f3_mut(),
            Self::Level5(value) => value.terrane_field_f3_mut(),
            Self::Level6(value) => value.terrane_field_f3_mut(),
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
}
#[derive(Clone)]
pub struct Level4Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
}
impl Level4Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level4 {
    Own(Level4Storage),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level4 {
    pub fn terrane_construct() -> Self { Self::Own(Level4Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level5(value) => value.terrane_field_f2(),
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level5(value) => value.terrane_field_f2_mut(),
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level5(value) => value.terrane_field_f3(),
            Self::Level6(value) => value.terrane_field_f3(),
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level5(value) => value.terrane_field_f3_mut(),
            Self::Level6(value) => value.terrane_field_f3_mut(),
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level5(value) => value.terrane_field_f4(),
            Self::Level6(value) => value.terrane_field_f4(),
            Self::Level7(value) => value.terrane_field_f4(),
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level5(value) => value.terrane_field_f4_mut(),
            Self::Level6(value) => value.terrane_field_f4_mut(),
            Self::Level7(value) => value.terrane_field_f4_mut(),
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
}
#[derive(Clone)]
pub struct Level5Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
}
impl Level5Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level5 {
    Own(Level5Storage),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level5 {
    pub fn terrane_construct() -> Self { Self::Own(Level5Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level6(value) => value.terrane_field_f3(),
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level6(value) => value.terrane_field_f3_mut(),
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level6(value) => value.terrane_field_f4(),
            Self::Level7(value) => value.terrane_field_f4(),
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level6(value) => value.terrane_field_f4_mut(),
            Self::Level7(value) => value.terrane_field_f4_mut(),
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level6(value) => value.terrane_field_f5(),
            Self::Level7(value) => value.terrane_field_f5(),
            Self::Level8(value) => value.terrane_field_f5(),
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level6(value) => value.terrane_field_f5_mut(),
            Self::Level7(value) => value.terrane_field_f5_mut(),
            Self::Level8(value) => value.terrane_field_f5_mut(),
            Self::Level9(value) => &mut value.f5,
        }
    }
}
#[derive(Clone)]
pub struct Level6Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
}
impl Level6Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level6 {
    Own(Level6Storage),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level6 {
    pub fn terrane_construct() -> Self { Self::Own(Level6Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level7(value) => value.terrane_field_f4(),
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level7(value) => value.terrane_field_f4_mut(),
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level7(value) => value.terrane_field_f5(),
            Self::Level8(value) => value.terrane_field_f5(),
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level7(value) => value.terrane_field_f5_mut(),
            Self::Level8(value) => value.terrane_field_f5_mut(),
            Self::Level9(value) => &mut value.f5,
        }
    }
    pub fn terrane_field_f6(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f6,
            Self::Level7(value) => value.terrane_field_f6(),
            Self::Level8(value) => value.terrane_field_f6(),
            Self::Level9(value) => &value.f6,
        }
    }
    pub fn terrane_field_f6_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f6,
            Self::Level7(value) => value.terrane_field_f6_mut(),
            Self::Level8(value) => value.terrane_field_f6_mut(),
            Self::Level9(value) => &mut value.f6,
        }
    }
}
#[derive(Clone)]
pub struct Level7Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
    pub f7: terrane_int_support::Int,
}
impl Level7Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
            f7: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level7 {
    Own(Level7Storage),
    Level8(Level8),
    Level9(Level9),
}
impl Level7 {
    pub fn terrane_construct() -> Self { Self::Own(Level7Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level8(value) => value.terrane_field_f5(),
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level8(value) => value.terrane_field_f5_mut(),
            Self::Level9(value) => &mut value.f5,
        }
    }
    pub fn terrane_field_f6(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f6,
            Self::Level8(value) => value.terrane_field_f6(),
            Self::Level9(value) => &value.f6,
        }
    }
    pub fn terrane_field_f6_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f6,
            Self::Level8(value) => value.terrane_field_f6_mut(),
            Self::Level9(value) => &mut value.f6,
        }
    }
    pub fn terrane_field_f7(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f7,
            Self::Level8(value) => value.terrane_field_f7(),
            Self::Level9(value) => &value.f7,
        }
    }
    pub fn terrane_field_f7_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f7,
            Self::Level8(value) => value.terrane_field_f7_mut(),
            Self::Level9(value) => &mut value.f7,
        }
    }
}
#[derive(Clone)]
pub struct Level8Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
    pub f7: terrane_int_support::Int,
    pub f8: terrane_int_support::Int,
}
impl Level8Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
            f7: terrane_int_support::Int::from(7_i128),
            f8: terrane_int_support::Int::from(8_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level8 {
    Own(Level8Storage),
    Level9(Level9),
}
impl Level8 {
    pub fn terrane_construct() -> Self { Self::Own(Level8Storage::terrane_construct()) }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level9(value) => &mut value.f5,
        }
    }
    pub fn terrane_field_f6(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f6,
            Self::Level9(value) => &value.f6,
        }
    }
    pub fn terrane_field_f6_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f6,
            Self::Level9(value) => &mut value.f6,
        }
    }
    pub fn terrane_field_f7(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f7,
            Self::Level9(value) => &value.f7,
        }
    }
    pub fn terrane_field_f7_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f7,
            Self::Level9(value) => &mut value.f7,
        }
    }
    pub fn terrane_field_f8(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f8,
            Self::Level9(value) => &value.f8,
        }
    }
    pub fn terrane_field_f8_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f8,
            Self::Level9(value) => &mut value.f8,
        }
    }
}
#[derive(Clone)]
pub struct Level9 {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
    pub f7: terrane_int_support::Int,
    pub f8: terrane_int_support::Int,
    pub f9: terrane_int_support::Int,
}
impl Level9 {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
            f7: terrane_int_support::Int::from(7_i128),
            f8: terrane_int_support::Int::from(8_i128),
            f9: terrane_int_support::Int::from(9_i128),
        }
    }
}
fn main() {
    let leaf: Level9 = Level9::terrane_construct();
    println!("{}{}{}", terrane_scalar_support::scalar_text(&(leaf.f0)), terrane_scalar_support::scalar_text(&(leaf.f5)), terrane_scalar_support::scalar_text(&(leaf.f9)));
    let mut root: Level0 = Level0::Level9((leaf).clone());
    println!("{}", terrane_scalar_support::scalar_text(&((root).terrane_field_f0().clone())));
    *(root).terrane_field_f0_mut() = terrane_int_support::Int::from(10_i128);
    println!("{}{}", terrane_scalar_support::scalar_text(&((root).terrane_field_f0().clone())), terrane_scalar_support::scalar_text(&(leaf.f0)));
}
