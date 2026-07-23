use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
pub type FxType = i128;
pub const FX_DIVISOR: i128 = 10000;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Fx {
    inner: i128,
}

impl Fx {
    pub const fn new(v: i64) -> Self {
        Self {
            inner: v as FxType * FX_DIVISOR,
        }
    }
    pub const fn new_i128(v: i128) -> Self {
        Self {
            inner: v as FxType * FX_DIVISOR,
        }
    }

    pub const fn new_f64(v: f64) -> Self {
        Self {
            inner: (v * FX_DIVISOR as f64).floor() as i128,
        }
    }
    pub const fn get_i64(&self) -> i64 {
        (self.inner / FX_DIVISOR) as i64
    }

    pub const fn get_f64(&self) -> f64 {
        self.inner as f64 / (FX_DIVISOR as f64)
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
            inner: ((self.inner * rhs.inner) / FX_DIVISOR),
        }
    }

    pub const fn op_div(&self, rhs: &Self) -> Self {
        Self {
            inner: (((FX_DIVISOR * FX_DIVISOR) / (rhs.inner)) * self.inner) / FX_DIVISOR,
        }
    }

    pub fn sqrt(&self) -> Self {
        Self::new_f64(self.get_f64().sqrt())
    }
    pub const fn get_inner(&self) -> i128 {
        self.inner
    }
    pub fn sin(&self) -> Self {
        fn approx_sin(fx: Fx) -> Fx {
            let mut out = Fx::new(0);
            for i in 0..15 {
                let mut div = 1;
                for j in 2..=2 * i + 1 {
                    div *= j;
                }
                let mut tmp = fx;
                for _ in 1..(i * 2 + 1) {
                    tmp *= fx;
                }
                if i % 2 == 0 {
                    out += tmp / Fx::new_i128(div);
                } else {
                    out -= tmp / Fx::new_i128(div);
                }
            }
            out
        }
        fn approx_cos(fx: Fx) -> Fx {
            let mut out = Fx::new(1);
            for i in 0..15 {
                let mut div = 1;
                for j in 2..=2 * i + 2 {
                    div *= j;
                }
                let mut tmp = fx;
                for _ in 0..=(i * 2) {
                    tmp *= fx;
                }
                if i % 2 == 0 {
                    out -= tmp / Fx::new_i128(div);
                } else {
                    out += tmp / Fx::new_i128(div);
                }
            }
            out
        }
        let sn_base = if *self > Fx::new(0) {
            Fx::new(1)
        } else {
            Fx::new(-1)
        };
        let base_zero = self.abs() % Self::TAU;
        let (base, sn) = if base_zero > Fx::PI {
            (base_zero - Fx::PI, sn_base * Fx::new(-1))
        } else {
            (base_zero, sn_base)
        };
        if base >= Fx::PI / Fx::new(2) {
            return approx_cos(base - Fx::PI / Fx::new(2)) * sn;
        } else {
            return approx_sin(base) * sn;
        }
    }

    pub const fn abs(&self) -> Self {
        Self {
            inner: self.inner.abs(),
        }
    }
    pub fn cos(&self) -> Self {
        -(*self - Self::PI / Self::new(2)).sin()
    }
    pub fn tan(&self) -> Self {
        todo!()
    }
    pub fn acos(&self) -> Self {
        todo!()
    }
    pub fn asin(&self) -> Self {
        todo!()
    }
    pub fn atan(&self) -> Self {
        todo!()
    }
    pub const PI: Self = Self::new_f64(std::f64::consts::PI);
    pub const TAU: Self = Self::new_f64(std::f64::consts::TAU);
}

impl std::ops::Neg for Fx {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { inner: -self.inner }
    }
}
impl From<i64> for Fx {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<f64> for Fx {
    fn from(value: f64) -> Self {
        Self::new_f64(value)
    }
}
impl From<Fx> for i64 {
    fn from(v: Fx) -> Self {
        v.get_i64()
    }
}

impl From<Fx> for f64 {
    fn from(v: Fx) -> Self {
        v.get_f64()
    }
}

impl std::ops::Add for Fx {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op_add(&rhs)
    }
}
impl std::ops::Sub for Fx {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.op_sub(&rhs)
    }
}

impl std::ops::Mul for Fx {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.op_mul(&rhs)
    }
}

impl std::ops::Div for Fx {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.op_div(&rhs)
    }
}
impl std::ops::AddAssign for Fx {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.op_add(&rhs);
    }
}

impl std::ops::SubAssign for Fx {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.op_sub(&rhs);
    }
}
impl std::ops::DivAssign for Fx {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.op_div(&rhs);
    }
}
impl std::ops::MulAssign for Fx {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.op_mul(&rhs);
    }
}
impl std::fmt::Display for Fx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_f64())
    }
}
impl std::fmt::Debug for Fx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_f64())
    }
}
impl std::ops::Rem for Fx {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner % rhs.inner,
        }
    }
}
impl std::ops::RemAssign for Fx {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: Fx,
    pub y: Fx,
    pub z: Fx,
}
impl Vec3 {
    pub const fn new(x: Fx, y: Fx, z: Fx) -> Self {
        Self { x, y, z }
    }

    pub const fn new_i64(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: Fx::new(x),
            y: Fx::new(y),
            z: Fx::new(z),
        }
    }

    pub const fn new_f64(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Fx::new_f64(x),
            y: Fx::new_f64(y),
            z: Fx::new_f64(z),
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

    pub const fn dot(&self, rhs: &Self) -> Fx {
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

    pub const fn scaled(&self, rhs: &Fx) -> Self {
        Self::new(self.x.op_mul(rhs), self.y.op_mul(rhs), self.z.op_mul(rhs))
    }
    pub const fn divided(&self, rhs: &Fx) -> Self {
        Self::new(self.x.op_div(rhs), self.y.op_div(rhs), self.z.op_div(rhs))
    }

    pub fn len(&self) -> Fx {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Self {
        self.scaled(&(Fx::new(1) / self.len()))
    }

    pub fn distance(&self, other: Self) -> Fx {
        (*self - other).len()
    }

    pub fn from_strs(s1: &str, s2: &str, s3: &str) -> Option<Self> {
        let s1a: String = s1.chars().filter(|i| *i != ',').collect();
        let s2a: String = s2.chars().filter(|i| *i != ',').collect();
        let s3a: String = s3.chars().filter(|i| *i != ',').collect();
        let Ok(xpos) = s1a.parse::<f64>() else {
            return None;
        };
        let Ok(ypos) = s2a.parse::<f64>() else {
            return None;
        };
        let Ok(zpos) = s3a.parse::<f64>() else {
            return None;
        };
        Some(Self::new_f64(xpos, ypos, zpos))
    }

    pub fn angle_between(&self, other: &Self) -> Fx {
        // a dot b = |a||b| cos(theta)
        let dot = self.dot(other);
        let ndot = dot / (self.len() * other.len());
        ndot.acos()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.op_add(&rhs)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.op_sub(&rhs)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<Fx> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Fx) -> Self {
        self.scaled(&rhs)
    }
}

impl std::ops::Div<Fx> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Fx) -> Self {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Self::new(x, y, z)
    }
}

impl std::ops::MulAssign<Fx> for Vec3 {
    fn mul_assign(&mut self, rhs: Fx) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<Fx> for Vec3 {
    fn div_assign(&mut self, rhs: Fx) {
        *self = *self / rhs;
    }
}

pub struct GraphConnection {
    pub to: usize,
    pub distance: Fx,
}

pub struct GraphNode<T> {
    pub connections: Vec<GraphConnection>,
    pub value: T,
}

pub struct Graph<T> {
    pub nodes: Vec<GraphNode<T>>,
}

impl<T> Graph<T> {
    pub fn get_node(&self, at: usize) -> &GraphNode<T> {
        &self.nodes[at]
    }

    pub fn get_node_mut(&mut self, at: usize) -> &mut GraphNode<T> {
        &mut self.nodes[at]
    }

    pub fn astar(
        &self,
        start: usize,
        end: usize,
        heuristic: impl Fn(&GraphNode<T>, usize) -> Fx,
    ) -> Option<Vec<usize>> {
        let reconstruct_path = |came_from: &HashMap<usize, usize>, current: usize| {
            let mut cur = current;
            let mut total_path = vec![current];
            while let Some(current) = came_from.get(&cur) {
                cur = *current;
                total_path.push(cur);
            }
            total_path
        };
        let find_min = |v: &HashSet<usize>, f_scores: &HashMap<usize, Fx>| {
            let mut mn = Fx::new(1000000000000000000);
            let mut min_idx = 0;
            for i in v {
                let Some(h) = f_scores.get(i) else {
                    continue;
                };
                if *h < mn {
                    min_idx = *i;
                    mn = *h;
                }
            }
            min_idx
        };
        let mut open_set = HashSet::new();
        open_set.insert(start);
        let mut came_from: HashMap<usize, usize> = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, Fx::new(0));
        let mut f_score = HashMap::new();
        f_score.insert(start, heuristic(&self.nodes[start], start));
        while !open_set.is_empty() {
            let current = find_min(&open_set, &f_score);
            if current == end {
                return Some(reconstruct_path(&came_from, current));
            }
            open_set.remove(&current);
            for i in &self.nodes[current].connections {
                let tentative_g_store = g_score[&current] + i.distance;
                if let Some(tmp) = g_score.get(&i.to) {
                    if tentative_g_store < *tmp {
                        came_from.insert(i.to, current);
                        g_score.insert(i.to, tentative_g_store);
                        f_score
                            .insert(i.to, tentative_g_store + heuristic(&self.nodes[i.to], i.to));
                        open_set.insert(i.to);
                    }
                } else {
                    came_from.insert(i.to, current);
                    g_score.insert(i.to, tentative_g_store);
                    f_score.insert(i.to, tentative_g_store + heuristic(&self.nodes[i.to], i.to));

                    open_set.insert(i.to);
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Orientation {
    up: Vec3,
    forward: Vec3,
}
impl Orientation {
    pub fn new(forward: Vec3, up: Vec3) -> Option<Self> {
        if forward.dot(&up) > 0.01.into() {
            return None;
        }
        Some(Self { up, forward })
    }

    pub fn forward(&self) -> Vec3 {
        self.forward
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn left(&self) -> Vec3 {
        self.forward.cross(&self.up)
    }

    #[must_use]
    pub fn rotated_pitch(&self, theta: f64) -> Self {
        let mat = Matrix3D::rotation_pitch(theta);
        let forward = mat.transform(self.forward);
        let up = mat.transform(self.up);
        Self { forward, up }
    }

    #[must_use]
    pub fn rotated_yaw(&self, theta: f64) -> Self {
        let mat = Matrix3D::rotation_yaw(theta);
        let forward = mat.transform(self.forward);
        let up = mat.transform(self.up);
        Self { forward, up }
    }

    #[must_use]
    pub fn rotated_roll(&self, theta: f64) -> Self {
        let mat = Matrix3D::rotation_roll(theta);
        let forward = mat.transform(self.forward);
        let up = mat.transform(self.up);
        Self { forward, up }
    }

    pub fn rotate_pitch(&mut self, theta: f64) {
        *self = self.rotated_pitch(theta);
    }

    pub fn rotate_yaw(&mut self, theta: f64) {
        *self = self.rotated_yaw(theta);
    }

    pub fn rotate_roll(&mut self, theta: f64) {
        *self = self.rotated_roll(theta);
    }

    #[must_use]
    pub fn rotated_toward(&self, toward: Vec3, max_radians: f64) -> Self {
        let mut out = *self;
        let mut min_delta = self.forward.angle_between(&toward);
        for dx in -314..=314 {
            for dy in -314..=314 {
                let s1 = self
                    .rotated_pitch(dy as f64 / 100.)
                    .rotated_yaw(dx as f64 / 100.);
                if s1.rotation_between(self) < max_radians.into() {
                    let rt = s1.forward.angle_between(&toward);
                    if rt < min_delta {
                        min_delta = rt;
                        out = s1;
                    }
                }
            }
        }
        for dx in -100..=100 {
            for dy in -100..=100 {
                let s1 = self
                    .rotated_pitch(dy as f64 / 10000.)
                    .rotated_yaw(dx as f64 / 10000.);
                if s1.rotation_between(self) < max_radians.into() {
                    let rt = s1.forward.angle_between(&toward);
                    if rt < min_delta {
                        min_delta = rt;
                        out = s1;
                    }
                }
            }
        }
        out
    }

    #[must_use]
    pub fn faced_towards(&self, toward: Vec3) -> Self {
        self.rotated_toward(toward, std::f64::consts::TAU * 8.)
    }

    pub fn rotation_between(&self, other: &Self) -> Fx {
        let a1 = self.forward.angle_between(&other.forward);
        let b1 = self.up.angle_between(&other.up);
        let at = a1 * a1 + b1 * b1;
        at.sqrt()
    }

    pub const fn identity() -> Self {
        Self {
            up: Vec3::new_i64(0, 0, 1),
            forward: Vec3::new_i64(1, 0, 0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Matrix3D {
    pub array: [[Fx; 3]; 3],
}

impl Matrix3D {
    pub fn transform(&self, on: Vec3) -> Vec3 {
        let base = [on.x, on.y, on.z];
        let mut out = [Fx::new(0), Fx::new(0), Fx::new(0)];
        for i in 0..3 {
            out[i] = self.array[i][0] * base[0]
                + self.array[i][1] * base[1]
                + self.array[i][2] * base[2];
        }
        Vec3::new(out[0], out[1], out[2])
    }

    pub fn rotation_pitch(theta: f64) -> Self {
        let mat: [[Fx; 3]; 3] = [
            [theta.cos().into(), 0.0.into(), theta.sin().into()],
            [0.into(), 1.into(), 0.into()],
            [(-theta.sin()).into(), 0.0.into(), theta.cos().into()],
        ];
        let m2 = Matrix3D { array: mat };
        m2
    }

    pub fn rotation_yaw(theta: f64) -> Self {
        let mat: [[Fx; 3]; 3] = [
            [theta.cos().into(), (-theta.sin()).into(), 0.into()],
            [theta.sin().into(), theta.cos().into(), 0.into()],
            [0.into(), 0.into(), 1.into()],
        ];
        let m2 = Matrix3D { array: mat };
        m2
    }

    pub fn rotation_roll(theta: f64) -> Self {
        let mat: [[Fx; 3]; 3] = [
            [1.into(), 0.into(), 0.into()],
            [0.into(), theta.cos().into(), (-theta.sin()).into()],
            [0.into(), theta.sin().into(), theta.cos().into()],
        ];
        let m2 = Matrix3D { array: mat };
        m2
    }
}
