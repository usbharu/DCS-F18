
use std::{collections::{HashMap, HashSet}, fs::File, io::Error, path::PathBuf, sync::{Arc, Mutex}};

use dcs_bios_const::{json_type::Function, parse_file};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Module{
    pub path: PathBuf,
    pub name: String,
    pub func: Option<HashMap<String,Function>>,
    pub categories: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Modules{
    pub modules: Arc<Mutex<HashMap<String,Module>>>
}

impl Module {
    fn set_func(&mut self, map: HashMap<String, Function>) {
        self.func = Some(map);
        let categories = self
            .func
            .as_ref()
            .unwrap()
            .values()
            .map(|p| p.category.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        self.categories = Some(categories);
    }

    pub fn is_none(&self) -> bool {
        self.func.is_none() || self.categories.is_none()
    }

    pub fn get_func(&mut self) -> Result<HashMap<String, Function>, Error> {
        let _ = &if self.func.is_some() {
            return Ok(self.func.clone().unwrap());
        };
        let file = File::open(&self.path)?;
        let func2 = parse_file(file)?;
        self.set_func(
            func2
                .into_iter()
                .map(|p| (p.identifier.clone(), p))
                .collect::<HashMap<_, _>>(),
        );
        Ok(self.func.clone().unwrap())
    }
}