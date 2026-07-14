use serde::{Deserialize, Serialize};

use crate::utils::{Fx, ObjRef, ObjectSet};
pub mod battleship;
pub mod utils;

#[derive(Clone, Serialize, Deserialize)]
pub struct DebugTester {
    pub x: i32,
    pub prev: ObjRef<DebugTester>,
}
impl Drop for DebugTester {
    fn drop(&mut self) {
        println!("dropped:{}", self.x);
    }
}
pub static OBJECTS: ObjectSet<DebugTester> = create_static_object_set!(DebugTester, 4096);
fn main() {
    let load = true;
    if !load {
        let objects = ObjectSet::new_dynamic(4096);
        let mut prev = ObjRef::new();
        for i in 0..100 {
            prev = objects.new_object(DebugTester {
                x: i,
                prev: prev.clone(),
            });
        }
        loop {
            let Ok(tmp) = prev.read_checked() else {
                break;
            };
            let t2 = tmp.get().prev.clone();
            println!("value:{}", tmp.get().x);
            drop(tmp);
            prev = t2;
        }
        std::fs::write("test.json", objects.save().unwrap()).unwrap();
    } else {
        let objects: ObjectSet<DebugTester> = ObjectSet::new_dynamic(4096);
        let str = std::fs::read_to_string("test.json").unwrap();
        objects.load(&str).unwrap();
    }
}
