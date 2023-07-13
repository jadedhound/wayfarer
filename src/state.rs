use serde::{Deserialize, Serialize};

/// Keeps a timestamp of when the creating a new pc is acceptable again
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct NewPCTimeout(pub f64);

#[derive(Serialize, Deserialize, Clone)]
pub struct PCBrief {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCList(Vec<PCBrief>);

impl PCList {
    pub fn get_all(&self) -> core::slice::Iter<PCBrief> {
        self.0.iter()
    }
    pub fn get(&self, id: u64) -> Option<&PCBrief> {
        self.0.iter().find(|pc| pc.id == id)
    }

    pub fn add(&mut self, name: String) {
        self.0.push(PCBrief {
            id: js_sys::Date::now() as u64,
            name,
        })
    }

    //pub fn remove(&mut self, id: u64) {
    //    if let Some(i) = self.0.iter().position(|pc| pc.id == id) {
    //        self.0.remove(i);
    //    };
    //}
}
