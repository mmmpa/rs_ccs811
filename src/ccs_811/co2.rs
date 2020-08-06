use core::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Co2(pub f32);

impl Add for Co2 {
    type Output = Co2;

    fn add(self, rhs: Self) -> Self::Output {
        Co2(self.0 + rhs.0)
    }
}

impl Co2 {
    pub fn div(&self, n: f32) -> Self {
        Co2(self.0 / n)
    }
}

impl ToString for Co2 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
