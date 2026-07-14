use std::{
    collections::VecDeque,
    fmt::Debug,
    marker::PhantomData,
    ops::DerefMut,
    ptr::null,
    sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

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
            inner: ((FX_DIVISOR * FX_DIVISOR) / (rhs.inner)) * (self.inner / FX_DIVISOR),
        }
    }

    pub fn sqrt(&self) -> Self {
        Self::new_f64(self.get_f64().sqrt())
    }
    pub const fn get_inner(&self) -> i128 {
        self.inner
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

#[derive(Serialize, Deserialize)]
pub struct ObjectShell<T> {
    value: Option<T>,
    generation: u64,
}
pub struct ObjectSet<T: 'static> {
    objects: StaticRef<[RwLock<ObjectShell<T>>]>,
    destructor_queue: Mutex<VecDeque<(u64, u64)>>,
}

impl<T> Drop for ObjectSet<T> {
    fn drop(&mut self) {
        for i in self.objects.get().iter() {
            _ = i.write().unwrap().value.take();
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjRef<T: 'static> {
    #[serde(serialize_with = "serialize_object_set_ref::<T, _>")]
    #[serde(deserialize_with = "deserialize_object_set_ref::<T, _>")]
    rf: Option<StaticRef<[RwLock<ObjectShell<T>>]>>,
    idx: u64,
    generation: u64,
}
impl<T: 'static> ObjectShell<T> {
    pub const fn new() -> Self {
        Self {
            value: None,
            generation: 0,
        }
    }
}

enum StaticRefInner<T: ?Sized + 'static> {
    Static(&'static T),
    Dynamic(Arc<T>),
}
#[repr(transparent)]
pub struct StaticRef<T: ?Sized + 'static> {
    inner: StaticRefInner<T>,
}

impl<T: ?Sized + 'static> StaticRef<T> {
    pub const fn new_static(val: &'static T) -> Self {
        Self {
            inner: StaticRefInner::Static(val),
        }
    }

    pub fn new_dynamic(val: Arc<T>) -> Self {
        Self {
            inner: StaticRefInner::Dynamic(val),
        }
    }

    pub fn get<'a>(&'a self) -> &'a T {
        match &self.inner {
            StaticRefInner::Static(ptr) => ptr,
            StaticRefInner::Dynamic(ptr) => ptr.as_ref(),
        }
    }
}

impl<T: ?Sized + 'static> Clone for StaticRef<T> {
    fn clone(&self) -> Self {
        match &self.inner {
            StaticRefInner::Dynamic(val) => Self {
                inner: StaticRefInner::Dynamic(val.clone()),
            },
            StaticRefInner::Static(val) => Self {
                inner: StaticRefInner::Static(*val),
            },
        }
    }
}

impl<T: 'static> ObjectSet<T> {
    pub const fn new_static(list: &'static [RwLock<ObjectShell<T>>]) -> Self {
        Self {
            objects: StaticRef::new_static(list),
            destructor_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn new_dynamic(count: usize) -> Self {
        let mut list = Vec::new();
        list.reserve_exact(count);
        for _ in 0..count {
            list.push(RwLock::new(ObjectShell::new()));
        }
        Self {
            objects: StaticRef::new_dynamic(list.into()),
            destructor_queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl<T: 'static> ObjectSet<T> {
    pub fn new_object(&self, v: T) -> ObjRef<T> {
        let objs = self.objects.get();
        for i in 0..objs.len() {
            let mut guard = match objs[i].try_write() {
                Ok(t) => t,
                Err(t) => match t {
                    std::sync::TryLockError::Poisoned(t) => t.into_inner(),
                    std::sync::TryLockError::WouldBlock => {
                        continue;
                    }
                },
            };
            if guard.value.is_none() {
                guard.generation = guard.generation.wrapping_add(1);
                guard.value = Some(v);
                return ObjRef {
                    rf: Some(self.objects.clone()),
                    idx: i as u64,
                    generation: guard.generation,
                };
            }
        }
        ObjRef {
            rf: Some(self.objects.clone()),
            idx: 0,
            generation: 0,
        }
    }

    pub fn delete_object_actual(&self, objr: ObjRef<T>) {
        let Some(guard) = self.objects.get().get(objr.idx as usize) else {
            return;
        };
        let mut guard2 = guard.write().unwrap();
        if guard2.generation == objr.generation {
            guard2.value = None;
        }
    }

    pub fn delete_object(&self, objr: ObjRef<T>) {
        self.destructor_queue
            .lock()
            .unwrap()
            .push_back((objr.idx, objr.generation));
    }

    pub fn garbage_collect(&self) {
        loop {
            let mut tmp = self.destructor_queue.lock().unwrap();
            let Some((idx, genr)) = tmp.pop_front() else {
                break;
            };
            drop(tmp);
            if let Some(obj) = self.objects.get().get(idx as usize) {
                let mut guard = obj.write().unwrap();
                if guard.generation == genr {
                    guard.value = None;
                }
            }
        }
    }
}
impl<T: 'static + Serialize + DeserializeOwned> ObjectSet<T> {
    pub fn save(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self.objects.get())
    }
    pub fn load(&self, str: &str) -> Result<(), serde_json::Error> {
        let t1 = &self.objects as *const _ as *const ();
        CURRENT_OBJECT_SET_REF.with(|i| {
            i.lock().unwrap().ptr = t1;
        });
        let value: Result<Vec<RwLock<ObjectShell<T>>>, _> = serde_json::from_str(str);
        CURRENT_OBJECT_SET_REF.with(|i| {
            i.lock().unwrap().ptr = null();
        });
        let value = value?;
        assert!(value.len() <= self.objects.get().len());
        for i in 0..value.len() {
            let mut guard = self.objects.get()[i].write().unwrap();
            let mut guard2 = value[i].write().unwrap();
            guard.value = guard2.value.take();
            guard.generation = guard2.generation;
        }
        for i in value.len()..self.objects.get().len() {
            let mut guard = self.objects.get()[i].write().unwrap();
            guard.generation = 0;
            guard.value = None;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! create_static_object_set {
    ($t:ty, $count:literal) => {
        const {
            static OBJECT_LIST: [std::sync::RwLock<crate::utils::ObjectShell<$t>>; $count] =
                [const { std::sync::RwLock::new(crate::utils::ObjectShell::new()) }; _];
            ObjectSet::new_static(&OBJECT_LIST)
        }
    };
}

pub struct ObjectSetDeserializizationRef {
    pub ptr: *const (),
}
thread_local! {
    pub static CURRENT_OBJECT_SET_REF:Mutex<ObjectSetDeserializizationRef> = Mutex::new(ObjectSetDeserializizationRef { ptr: std::ptr::null()});
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MarkerOptionStaticRefRwLockObjectShell<T> {
    __phantom: PhantomData<T>,
}

pub fn deserialize_object_set_ref<'de, T, D>(
    _v: D,
) -> Result<Option<StaticRef<[RwLock<ObjectShell<T>>]>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let tmp = Option::<MarkerOptionStaticRefRwLockObjectShell<T>>::deserialize(_v)?;
    assert!(tmp.is_none());
    CURRENT_OBJECT_SET_REF.with(|v| {
        let guard = v.lock().unwrap();

        if guard.ptr.is_null() {
            return Ok(None);
        }
        let rf = unsafe {
            let pt = (*(guard.ptr as *const StaticRef<[RwLock<ObjectShell<T>>]>)).clone();
            pt
        };
        Ok::<_, D::Error>(Some(rf))
    })
}

pub fn serialize_object_set_ref<T, S>(
    _sv: &Option<StaticRef<[std::sync::RwLock<ObjectShell<T>>]>>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    Option::<MarkerOptionStaticRefRwLockObjectShell<T>>::None.serialize(ser)
}

impl<T: 'static> ObjRef<T> {
    pub const fn new() -> Self {
        Self {
            rf: None,
            idx: 0,
            generation: 0,
        }
    }
    fn get_object_set(&self) -> Result<&[RwLock<ObjectShell<T>>], ObjectGetError> {
        if let Some(t) = self.rf.as_ref() {
            return Ok(t.get());
        } else {
            return Err(ObjectGetError::InvalidReference);
        }
    }
    pub fn read<'a>(&'a self) -> ObjRead<'a, T> {
        let tmp = self
            .rf
            .as_ref()
            .unwrap()
            .get()
            .get(self.idx as usize)
            .unwrap();
        let guard = tmp.read().unwrap();
        assert!(guard.value.is_some() && guard.generation == self.generation);
        ObjRead { guard }
    }

    pub fn write<'a>(&'a self) -> ObjWrite<'a, T> {
        let tmp = self
            .rf
            .as_ref()
            .unwrap()
            .get()
            .get(self.idx as usize)
            .unwrap();
        let guard = tmp.write().unwrap();
        assert!(guard.value.is_some() && guard.generation == self.generation);
        ObjWrite { guard }
    }

    pub fn read_checked<'a>(&'a self) -> Result<ObjRead<'a, T>, ObjectGetError> {
        let Some(tmp) = self.get_object_set()?.get(self.idx as usize) else {
            return Err(ObjectGetError::InvalidReference);
        };
        let guard = tmp.read().unwrap();
        if !(guard.value.is_some() && guard.generation == self.generation) {
            return Err(ObjectGetError::InvalidReference);
        }
        Ok(ObjRead { guard })
    }

    pub fn write_checked<'a>(&'a self) -> Result<ObjWrite<'a, T>, ObjectGetError> {
        let Some(tmp) = self.get_object_set()?.get(self.idx as usize) else {
            return Err(ObjectGetError::InvalidReference);
        };
        let guard = tmp.write().unwrap();
        if !(guard.value.is_some() && guard.generation == self.generation) {
            return Err(ObjectGetError::InvalidReference);
        }
        Ok(ObjWrite { guard })
    }

    pub fn try_read<'a>(&'a self) -> Result<ObjRead<'a, T>, ObjectGetError> {
        let Some(tmp) = self.get_object_set()?.get(self.idx as usize) else {
            return Err(ObjectGetError::InvalidReference);
        };
        let guard = match tmp.try_read() {
            Ok(t) => t,
            Err(t) => match t {
                std::sync::TryLockError::Poisoned(t) => t.into_inner(),
                std::sync::TryLockError::WouldBlock => {
                    return Err(ObjectGetError::WasLocked);
                }
            },
        };
        if !(guard.value.is_some() && guard.generation == self.generation) {
            return Err(ObjectGetError::InvalidReference);
        }
        Ok(ObjRead { guard })
    }

    pub fn try_write<'a>(&'a self) -> Result<ObjWrite<'a, T>, ObjectGetError> {
        let Some(tmp) = self.get_object_set()?.get(self.idx as usize) else {
            return Err(ObjectGetError::InvalidReference);
        };
        let guard = match tmp.try_write() {
            Ok(t) => t,
            Err(t) => match t {
                std::sync::TryLockError::Poisoned(t) => t.into_inner(),
                std::sync::TryLockError::WouldBlock => {
                    return Err(ObjectGetError::WasLocked);
                }
            },
        };
        if !(guard.value.is_some() && guard.generation == self.generation) {
            return Err(ObjectGetError::InvalidReference);
        }
        Ok(ObjWrite { guard })
    }
}
impl<T: 'static + Debug> Debug for ObjRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.try_read() {
            write!(
                f,
                "ObjRef<{}>{{ptr:{:#?}}}",
                std::any::type_name::<T>(),
                t.get()
            )
        } else {
            write!(
                f,
                "ObjRef<{}>{{ptr:null/could not access}}",
                std::any::type_name::<T>(),
            )
        }
    }
}
impl<T: 'static + std::fmt::Display> std::fmt::Display for ObjRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.try_read() {
            write!(f, "{}", t.get())
        } else {
            write!(f, "null")
        }
    }
}
impl<T: 'static> Default for ObjRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ObjRead<'a, T> {
    guard: RwLockReadGuard<'a, ObjectShell<T>>,
}

impl<'a, T> ObjRead<'a, T> {
    pub fn get(&self) -> &T {
        self.guard.value.as_ref().unwrap()
    }
}

pub struct ObjWrite<'a, T> {
    guard: RwLockWriteGuard<'a, ObjectShell<T>>,
}

impl<'a, T> ObjWrite<'a, T> {
    pub fn get(&self) -> &T {
        self.guard.value.as_ref().unwrap()
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.guard.value.as_mut().unwrap()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ObjectGetError {
    WasLocked,
    InvalidReference,
}
impl std::fmt::Display for ObjectGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for ObjectGetError {}
