use core::str;
use dcs_bios::{mem::MemoryMap, DcsBios, DcsBiosImpl};
use dcs_bios_const::json_type::{Function, Output, Type};
use modules::{Module, Modules};
use serde::{Deserialize, Serialize};
use source::{Source, UdpSource};
use tokio::time;
use std::{
    collections::HashMap,
    env,
    net::Ipv4Addr,
    ops::RangeInclusive,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex}, time::Duration,
};
use tauri::{App, Emitter, Manager, State};
mod modules;
mod source;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn modules(modules: State<'_, Modules>) -> Result<Vec<String>, String> {
    let modules = modules.modules.lock().map_err(|e| e.to_string())?;
    Ok(modules.keys().cloned().collect::<Vec<String>>())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceConfig {
    pub addr: Arc<std::sync::Mutex<Option<String>>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Listening {
    pub ids: Arc<std::sync::Mutex<Vec<Function>>>,
}

#[tauri::command(async)]
async fn subscribe(
    listening: State<'_, Listening>,
    modules: State<'_, Modules>,
    module_name: &str,
    id: &str,
) -> Result<(), String> {
    let mut guard = listening.ids.lock().unwrap();
    if guard.iter().find(|p| p.identifier == id).is_some() {
        return Ok(());
    };
    let mut modules = modules.modules.lock().map_err(|e| e.to_string())?;
    let module = modules
        .iter_mut()
        .find(|p| p.0 == module_name)
        .ok_or("".to_string())?
        .1;
    if module.is_none() {
        let _ = module.get_func();
    };

    match &module.func {
        Some(v) => {
            let func = v
                .iter()
                .find(|p| p.0 == id)
                .ok_or("".to_string())?
                .1
                .clone();
            guard.push(func);
        }

        None => {}
    };
    Ok(())
}

#[tauri::command]
fn get_subscribed(listening: State<'_, Listening>) -> Vec<Function> {
    let guard = listening.ids.lock().unwrap();
    guard.clone()
}

#[tauri::command(async)]
async fn categories(modules: State<'_, Modules>, module_name: &str) -> Result<Vec<String>, String> {
    let mut modules = modules.modules.lock().map_err(|e| e.to_string())?;
    let module = modules
        .iter_mut()
        .find(|p| p.0 == module_name)
        .ok_or("".to_string())?
        .1;
    if module.is_none() {
        let _ = module.get_func();
    }
    Ok(module.categories.clone().unwrap())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Data {
    IntegerData {
        id: String,
        address: u16,
        value: u16,
    },
    StringData {
        id: String,
        address: u16,
        value: String,
    },
}

impl Data {
    pub fn address(&self) -> u16 {
        match self {
            Data::IntegerData { id: _,address, value: _ } => *address,
            Data::StringData { id:_, address, value: _ } => *address,
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Data::IntegerData { id,address:_, value: _ } => id,
            Data::StringData { id, address:_, value: _ } => id,
        }
    }
}

#[tauri::command]
fn connect(
    source: State<'_, SourceConfig>,
    listening: State<'_, Listening>,
    app_handle: tauri::AppHandle,
    bind: &str,
    address: &str,
    interface: &str,
) -> Result<(), String> {
    println!("bind: {} addr: {} interface: {}", bind, address, interface);

    *(source.addr.lock().unwrap()) = Some(bind.to_string());

    let source_c = bind.to_string();
    let udp_source = UdpSource::from_addr(
        bind,
        &Ipv4Addr::from_str(address).map_err(|_| "Addressが不正です".to_string())?,
        &Ipv4Addr::from_str(interface).map_err(|_| "Interfaceが不正です".to_string())?,
    )
    .map_err(|e| e.to_string())?;

    let mem_map = VecMemoryMap::new([0; 65536].to_vec());
    udp_source.setup().unwrap();
    let mut dcs_bios = DcsBiosImpl::new(udp_source, mem_map);

    tauri::async_runtime::spawn({
        let source = source.addr.clone();
        let listening = listening.ids.clone();
        async move {
            let mut counter: u16 = 0;

            let mut list: Vec< Output> = vec![];

            loop {
                if counter % 1000 == 0 {
                    let a = source.lock().unwrap();
                    let a = a.as_ref().unwrap();

                    list = listening
                        .lock()
                        .unwrap()
                        .iter()
                        .flat_map(|x| {
                            x.outputs
                                .iter()
                                .map(|f| f.clone())
                                .collect::<Vec<Output>>()
                        })
                        .collect::<Vec<Output>>();
                    if source_c != *a {
                        println!("break!");
                        break;
                    }
                }
                counter = counter.wrapping_add(1);

                let packets = {
                    let dcs_bios = &mut dcs_bios;
                    let Ok(packets) = dcs_bios.read_packet() else {
                                time::sleep(Duration::from_micros(100)).await;
                        continue;
                    };
                    packets
                };

                let mut vec = vec![];

                for ele in packets {
                    vec.append(
                        &mut list
                            .iter()
                            .filter_map(move | o| {
                                if (ele.address..=(ele.address + (ele.length - 1))).contains(&o.address) {
                                    return Some(o);
                                } else {
                                    return None;
                                }
                            })
                            .collect::<Vec<&Output>>(),
                    );
                }

                let mut vec2 = vec![];

                for it in vec {
                    vec2.push(match it.r#type {
                        Type::integer => Data::IntegerData {
                            id: format!("{}_{}_{}_{}",it.address,it.r#type,it.mask.unwrap(),it.shift_by.unwrap()),
                            address: it.address,
                            value: dcs_bios
                                .get_self_integer(
                                    it.address,
                                    it.mask.unwrap(),
                                    it.shift_by.unwrap(),
                                )
                                .unwrap(),
                        },
                        Type::string => Data::StringData {
                            id: format!("{}_{}_{}",it.address,it.r#type,it.max_length.unwrap()),
                            address: it.address,
                            value: dcs_bios
                                .get_self_string(it.address, it.max_length.unwrap())
                                .unwrap()
                                .to_string(),
                        },
                    });
                }

                if vec2.is_empty() {
                    continue;
                }

                vec2.sort_by(|a,b| a.id().cmp(b.id()));
                vec2.dedup_by(|a,b| a.id() == b.id() );
                println!("{:?}", vec2);
                app_handle.emit("data", vec2).unwrap();
            }
        }
    });
    Ok(())
}

#[tauri::command(async)]
async fn ids(
    modules: State<'_, Modules>,
    module_name: &str,
    category_name: &str,
) -> Result<Vec<String>, String> {
    let mut modules = modules.modules.lock().map_err(|e| e.to_string())?;
    let module = modules
        .iter_mut()
        .find(|p| p.0 == module_name)
        .ok_or("".to_string())?
        .1;
    if module.is_none() {
        let _ = module.get_func();
    }
    Ok(match &module.func {
        Some(v) => v
            .iter()
            .filter(|p| p.1.category == category_name)
            .map(|o| o.0.clone())
            .collect(),
        None => return Err("".to_string()),
    })
}

fn setup_modules(app: &mut App) {
    let Some(user_profile) = env::var_os("USERPROFILE") else {
        println!("USERPROFILE environment variable not found.");
        app.manage(Modules {
            modules: Arc::new(std::sync::Mutex::new(HashMap::new())),
        });
        return;
    };
    let dcs_path: PathBuf = [
        user_profile,
        "Saved Games".into(),
        "DCS".into(),
        "Scripts".into(),
        "DCS-BIOS".into(),
        "doc".into(),
        "json".into(),
    ]
    .iter()
    .collect();
    println!("JSON Path: {:?}", dcs_path);

    let Ok(dir) = dcs_path.read_dir() else {
        println!("Cannot read dir {:?}", dcs_path);
        return;
    };

    let modules = dir
        .filter_map(|it| {
            let Ok(it) = it else {
                println!("File read error: {:?}", it);
                return None;
            };
            let path = it.path();
            let file_name = path.file_name()?;
            let name = file_name
                .to_os_string()
                .into_string()
                .expect("OsString Error");
            Some((
                name.clone(),
                Module {
                    path: it.path(),
                    name,
                    func: None,
                    categories: None,
                },
            ))
        })
        .collect::<HashMap<String, Module>>();

    app.manage(Modules {
        modules: Arc::new(std::sync::Mutex::new(modules)),
    });
}

#[tauri::command]
fn unsubscribe(listening: State<'_, Listening>, id: &str) -> () {
    let mut listening = listening.ids.lock().unwrap();

    if let Some(index) = listening.iter().position(|p| p.identifier == id) {
        listening.remove(index);
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(SourceConfig {
                addr: Arc::new(Mutex::new(None)),
            });
            app.manage(Listening {
                ids: Arc::new(Mutex::new(vec![])),
            });
            setup_modules(app);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            modules,
            categories,
            ids,
            connect,
            subscribe,
            get_subscribed,
            unsubscribe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug)]
pub struct VecMemoryMap {
    mem: Vec<u8>,
}

impl VecMemoryMap {
    pub fn new(map: Vec<u8>) -> VecMemoryMap {
        VecMemoryMap { mem: map }
    }
}

impl MemoryMap for VecMemoryMap {
    fn write(
        &mut self,
        address: u16,
        data: &[u8],
    ) -> Result<RangeInclusive<u16>, dcs_bios::error::Error> {
        for (index, ele) in data.iter().enumerate() {
            self.mem[address as usize + index] = *ele;
        }
        Ok(address..=(address + (data.len() as u16 - 1)))
    }

    fn read(&self, range: RangeInclusive<u16>) -> Option<&[u8]> {
        self.mem
            .get((*range.start() as usize)..=(*range.end() as usize))
    }
}
