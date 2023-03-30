#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Time(f64);

impl Time {
    pub fn zero() -> Self {
        return Self(0.0);
    }

    pub fn from_seconds(value: impl Into<f64>) -> Self {
        return Time(value.into());
    }

    pub fn from_ms(value: impl Into<f64>) -> Self {
        return Time(value.into() / 1000.0);
    }

    pub fn to_seconds(&self) -> f64 {
        return self.0;
    }

    pub fn to_ms(&self) -> u64 {
        return (self.0 * 1000.0).round() as u64;
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        return Time(self.0 + rhs.0);
    }
}

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        return Time(self.0 - rhs.0);
    }
}

impl std::ops::Mul for Time {
    type Output = Time;

    fn mul(self, rhs: Self) -> Self::Output {
        return Time(self.0 * rhs.0);
    }
}

impl std::ops::Div for Time {
    type Output = Time;

    fn div(self, rhs: Self) -> Self::Output {
        return Time(self.0 / rhs.0);
    }
}