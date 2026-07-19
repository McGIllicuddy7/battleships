#[derive(Serialize, Deserialize)]
pub struct ObjectShell<T> {
    value: Option<T>,
    generation: u64,
}
pub struct ObjectSet<T: 'static> {
    creation_guard: Mutex<()>,
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
            creation_guard: Mutex::new(()),
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
            creation_guard: Mutex::new(()),
            objects: StaticRef::new_dynamic(list.into()),
            destructor_queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl<T: 'static> ObjectSet<T> {
    pub fn new_object(&self, v: T) -> ObjRef<T> {
        let _guard = self.creation_guard.lock().unwrap();
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
        let _guard = self.creation_guard.lock().unwrap();
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
        let _guard = self.creation_guard.lock();
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

    pub fn for_each(&self, mut func: impl FnMut(&T, ObjRef<T>)) {
        let objects = self.objects.get();
        objects.iter().enumerate().for_each(|(idx, i)| {
            let t = i.read().unwrap();
            let genr = t.generation;
            if let Some(rf) = t.value.as_ref() {
                let rf2 = ObjRef {
                    idx: idx as u64,
                    generation: genr,
                    rf: Some(self.objects.clone()),
                };
                func(rf, rf2);
            }
        });
    }

    pub fn for_each_mut(&self, mut func: impl FnMut(&mut T, ObjRef<T>)) {
        let objects = self.objects.get();
        objects.iter().enumerate().for_each(|(idx, i)| {
            let mut t = i.write().unwrap();
            let genr = t.generation;
            if let Some(rf) = t.value.as_mut() {
                let rf2 = ObjRef {
                    idx: idx as u64,
                    generation: genr,
                    rf: Some(self.objects.clone()),
                };
                func(rf, rf2);
            }
        });
    }
}
impl<T: 'static + Serialize + DeserializeOwned> ObjectSet<T> {
    pub fn save(&self) -> Result<String, serde_json::Error> {
        let _guard = self.creation_guard.lock().unwrap();
        let mut max_idx = 0;
        for (idx, i) in self.objects.get().iter().enumerate() {
            if let Ok(t) = i.try_read() {
                if t.value.is_some() {
                    max_idx = idx;
                }
            } else {
                max_idx = idx;
            }
        }
        if self.objects.get().len() == 0 {
            serde_json::to_string_pretty(self.objects.get())
        } else {
            serde_json::to_string_pretty(&self.objects.get()[0..=max_idx])
        }
    }
    pub fn load(&self, str: &str) -> Result<(), serde_json::Error> {
        let _guard = self.creation_guard.lock().unwrap();
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

impl<T: 'static + Send + Sync> ObjectSet<T> {
    pub fn for_each_par(&self, func: impl Fn(&T, ObjRef<T>) + Send + Sync) {
        let objects = self.objects.get();
        objects
            .iter()
            .enumerate()
            .par_bridge()
            .for_each(|(idx, i)| {
                let t = i.read().unwrap();
                let genr = t.generation;
                if let Some(rf) = t.value.as_ref() {
                    let rf2 = ObjRef {
                        idx: idx as u64,
                        generation: genr,
                        rf: Some(self.objects.clone()),
                    };
                    func(rf, rf2);
                }
            });
    }

    pub fn for_each_mut_par(&self, func: impl Fn(&mut T, ObjRef<T>) + Send + Sync) {
        let objects = self.objects.get();
        objects
            .iter()
            .enumerate()
            .par_bridge()
            .for_each(|(idx, i)| {
                let mut t = i.write().unwrap();
                let genr = t.generation;
                if let Some(rf) = t.value.as_mut() {
                    let rf2 = ObjRef {
                        idx: idx as u64,
                        generation: genr,
                        rf: Some(self.objects.clone()),
                    };
                    func(rf, rf2);
                }
            });
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
