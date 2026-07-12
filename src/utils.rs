pub const FIXED_DIVISOR: i64 = 10000;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed {
    inner: i64,
}

impl Fixed {
    pub const fn new(v: i64) -> Self {
        Self {
            inner: v * FIXED_DIVISOR,
        }
    }

    pub const fn new_f64(v: f64) -> Self {
        Self {
            inner: (v * FIXED_DIVISOR as f64).floor() as i64,
        }
    }
    pub const fn get_i64(&self) -> i64 {
        self.inner / FIXED_DIVISOR
    }

    pub const fn get_f64(&self) -> f64 {
        self.inner as f64 / (FIXED_DIVISOR as f64)
    }

    pub const fn op_add(&self, rhs: &Self) -> Self {
        Self {
            inner: self.inner + rhs.inner,
        }
    }

    pub const fn op_sub(&self, rhs: &Self) -> Self {
        Self {
            inner: self.inner - rhs.inner,
        }
    }

    pub const fn op_mul(&self, rhs: &Self) -> Self {
        Self {
            inner: ((self.inner * rhs.inner) / FIXED_DIVISOR),
        }
    }

    pub const fn op_div(&self, rhs: &Self) -> Self {
        Self {
            inner: (self.inner / rhs.inner),
        }
    }
}

impl From<i64> for Fixed {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<f64> for Fixed {
    fn from(value: f64) -> Self {
        Self::new_f64(value)
    }
}
impl Into<i64> for Fixed {
    fn into(self) -> i64 {
        self.get_i64()
    }
}

impl Into<f64> for Fixed {
    fn into(self) -> f64 {
        self.get_f64()
    }
}

impl std::ops::Add for Fixed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op_add(&rhs)
    }
}
impl std::ops::Sub for Fixed {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.op_sub(&rhs)
    }
}

impl std::ops::Mul for Fixed {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.op_mul(&rhs)
    }
}

impl std::ops::Div for Fixed {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.op_div(&rhs)
    }
}
impl std::ops::AddAssign for Fixed {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.op_add(&rhs);
    }
}

impl std::ops::SubAssign for Fixed {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.op_sub(&rhs);
    }
}
impl std::ops::DivAssign for Fixed {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.op_div(&rhs);
    }
}
impl std::ops::MulAssign for Fixed {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.op_mul(&rhs);
    }
}
impl std::fmt::Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_f64())
    }
}
impl std::fmt::Debug for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_f64())
    }
}
