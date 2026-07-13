pub type FixedType = i128;
pub const FIXED_DIVISOR: i128 = 10000;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed {
    inner: i128,
}

impl Fixed {
    pub const fn new(v: i64) -> Self {
        Self {
            inner: v as FixedType * FIXED_DIVISOR,
        }
    }

    pub const fn new_f64(v: f64) -> Self {
        Self {
            inner: (v * FIXED_DIVISOR as f64).floor() as i128,
        }
    }
    pub const fn get_i64(&self) -> i64 {
        (self.inner / FIXED_DIVISOR) as i64
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

    pub fn sqrt(&self) -> Self {
        Self::new_f64(self.get_f64().sqrt())
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
impl From<Fixed> for i64 {
    fn from(v: Fixed) -> Self {
        v.get_i64()
    }
}

impl From<Fixed> for f64 {
    fn from(v: Fixed) -> Self {
        v.get_f64()
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

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
}
impl Vector {
    pub const fn new(x: Fixed, y: Fixed, z: Fixed) -> Self {
        Self { x, y, z }
    }

    pub const fn new_i64(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: Fixed::new(x),
            y: Fixed::new(y),
            z: Fixed::new(z),
        }
    }

    pub const fn new_f64(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Fixed::new_f64(x),
            y: Fixed::new_f64(y),
            z: Fixed::new_f64(z),
        }
    }

    pub const fn op_add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.op_add(&rhs.x),
            y: self.y.op_add(&rhs.y),
            z: self.z.op_add(&rhs.z),
        }
    }

    pub const fn op_sub(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.op_sub(&rhs.x),
            y: self.y.op_sub(&rhs.y),
            z: self.z.op_sub(&rhs.z),
        }
    }

    pub const fn dot(&self, rhs: &Self) -> Fixed {
        let a = self.x.op_mul(&rhs.x);
        let b = self.y.op_mul(&rhs.y);
        let c = self.z.op_mul(&rhs.z);
        a.op_add(&b.op_add(&c))
    }

    pub const fn cross(&self, rhs: &Self) -> Self {
        let x = (self.y.op_mul(&rhs.z)).op_sub(&self.z.op_mul(&rhs.y));
        let y = (self.z.op_mul(&rhs.x)).op_sub(&self.x.op_mul(&rhs.z));
        let z = (self.x.op_mul(&rhs.y)).op_sub(&self.y.op_mul(&rhs.x));
        Self::new(x, y, z)
    }

    pub const fn scaled(&self, rhs: &Fixed) -> Self {
        Self::new(self.x.op_mul(rhs), self.y.op_mul(rhs), self.z.op_mul(rhs))
    }
    pub const fn divided(&self, rhs: &Fixed) -> Self {
        Self::new(self.x.op_div(rhs), self.y.op_div(rhs), self.z.op_div(rhs))
    }

    pub fn len(&self) -> Fixed {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Self {
        self.scaled(&(Fixed::new(1) / self.len()))
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.op_add(&rhs)
    }
}
impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.op_sub(&rhs)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<Fixed> for Vector {
    type Output = Self;
    fn mul(self, rhs: Fixed) -> Self {
        self.scaled(&rhs)
    }
}

impl std::ops::Div<Fixed> for Vector {
    type Output = Self;
    fn div(self, rhs: Fixed) -> Self {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Self::new(x, y, z)
    }
}

impl std::ops::MulAssign<Fixed> for Vector {
    fn mul_assign(&mut self, rhs: Fixed) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<Fixed> for Vector {
    fn div_assign(&mut self, rhs: Fixed) {
        *self = *self / rhs;
    }
}

pub type Fx64 = Fixed;
pub type Vec3 = Vector;
