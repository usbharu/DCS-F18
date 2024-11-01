mod data;
mod source;

use crate::data::Data;
use crate::data::Data::*;
use crate::source::{Source, UdpSource};
use core::str;
use dcs_bios::{parse_packet, DcsBiosPacket};
use dcs_bios_const_generator::{parse_file, Function, Output, Type};
use serde::{Deserialize, Serialize};
use source::TcpSource;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Error;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, vec};
use tauri::{Emitter, Manager, State};
use tokio::time;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(source: State<'_, SourceConfig>, name: &str) -> String {
    println!("{:?}", source.addr.lock().unwrap());
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn setup_socket(
    source: State<'_, SourceConfig>,
    listening: State<'_, Listening>,
    app_handle: tauri::AppHandle,
    bind: &str,
    addr: &str,
    interface: &str,
) -> Result<(), String> {
    println!("bind: {} addr: {} interface: {}", bind, addr, interface);

    *(source.addr.lock().unwrap()) = Some(bind.to_string());

    let source_c = bind.to_string();
    // let mut udp_source = UdpSource::from_addr(
    //     bind,
    //     &Ipv4Addr::from_str(addr).map_err(|_| "Addressが不正です".to_string())?,
    //     &Ipv4Addr::from_str(interface).map_err(|_| "Interfaceが不正です".to_string())?,
    // )

    let mut udp_source = TcpSource::from_addr(bind).map_err(|e| e.to_string())?;
    tauri::async_runtime::spawn({
        let source = source.addr.clone();
        let listening = listening.ids.clone();
        async move {
            let mut counter: u16 = 0;
            udp_source.setup().unwrap();
            let mut list: HashMap<u16, Output> = HashMap::new();
            loop {
                if counter % 100 == 0 {
                    let a = source.lock().unwrap();
                    let a = a.as_ref().unwrap();

                    list = listening
                        .lock()
                        .unwrap()
                        .iter()
                        .flat_map(|x| {
                            x.outputs
                                .iter()
                                .map(|f| (f.address, f.clone()))
                                .collect::<HashMap<u16, Output>>()
                        })
                        .collect::<HashMap<u16, Output>>();

                    println!("{:?}", list);
                    if source_c != *a {
                        println!("break!");
                        break;
                    }
                }
                counter = counter.wrapping_add(1);

                match udp_source.read() {
                    Ok(v) => {
                        let mut map = vec![];

                        let mut parser = dcs_bios::parser::ProtocolParser::new(|address, data| {
                            if (address[0] == 1078) {
                                // println!("{:?}", (data[0].to_le() & 0xfe00) >> 9);
                            }
                        });
                        for i in 0..v.len() {
                            match parser.process_char(v[i]) {
                                Some(r) => {
                                    // println!("address= {}",r.address) ;
                                    if !list.contains_key(&r.address) {
                                        continue;
                                    };
                                    let func = match list.iter().find(|(a, _)| **a == r.address) {
                                        Some(v) => v.1,
                                        None => {
                                            continue;
                                        }
                                    };
                                    map.push(match func.r#type {
                                        Type::integer => IntegerData {
                                            address: r.address,
                                            value: u16::from_le_bytes([r.data[30],r.data[31]]),
                                        },
                                        Type::string => StringData {
                                            address: r.address,
                                            value: str::from_utf8(&r.data).unwrap_or("<EEROR>").to_string(),
                                        },
                                    });
                                
                                }
                                None => {}
                            }
                        }

                        if map.is_empty() {
                            continue;
                        }
                        println!("{:?}", map);
                        app_handle.emit("data", map).unwrap();

                        // let map = DcsBiosPacket::new(&v).filter_map(|x| {
                        //     // if !list.contains_key(&x.address) {
                        //     //     return None;
                        //     // }

                        //     //todo functionからtypeをみてintegerdataとstringdataに入れる

                        //     let func = match list.iter().find(|(a,_)| {**a == x.address}) {
                        //         Some(v) => v.1,
                        //         None => {
                        //             return None;
                        //         }
                        //     };
                        //     Some(match func.r#type {
                        //         Type::integer => IntegerData {
                        //             address: x.address,
                        //             value: u16::from_le_bytes([x.data[0], x.data[1]]),
                        //         },
                        //         Type::string => StringData {
                        //             address: x.address,
                        //             value: str::from_utf8(x.data).unwrap_or("").to_string(),
                        //         },
                        //     })
                        // }).collect::<Vec<Data>>();
                        // if map.is_empty() {
                        //     continue;
                        // }
                        // println!("{:?}",map);
                        // app_handle.emit("data", map).unwrap();

                        // match parse_packet(&*v) {
                        //     None => {}
                        //     Some(v) => {
                        //         let mut map: Vec<Data> = v
                        //             .iter()
                        //             .filter_map(|x| {
                        //                 if x.address == 0 {
                        //                     return None;
                        //                 }
                        //                 if !list.contains_key(&x.address) {
                        //                     return None;
                        //                 }

                        //                 //todo functionからtypeをみてintegerdataとstringdataに入れる

                        //                 let func = match list.get(&x.address) {
                        //                     Some(v) => v,
                        //                     None => {
                        //                         return None;
                        //                     }
                        //                 };
                        //                 Some(match func.r#type {
                        //                     Type::integer => IntegerData {
                        //                         address: x.address,
                        //                         value: u16::from_le_bytes([x.data[0], x.data[1]]),
                        //                     },
                        //                     Type::string => StringData { address: x.address, value: str::from_utf8(x.data).unwrap().to_string() }
                        //                 })
                        //             })
                        //             .collect::<Vec<Data>>();
                        //         if map.is_empty() {
                        //             continue;
                        //         }

                        //         map.sort_by_key(|f| f.address());
                        //         map.dedup_by_key(|f| f.address());
                        //         println!("{:?}", map);
                        //         app_handle.emit("data", map).unwrap();
                        //     }
                        // };
                    }
                    Err(_) => {
                        time::sleep(Duration::from_micros(100)).await;
                    }
                };
            }
        }
    });
    Ok(())
}

#[tauri::command]
fn modules(modules: State<'_, Modules2>) -> Result<Vec<String>, String> {
    let modules = modules.modules.lock().map_err(|e| e.to_string())?;
    Ok(modules.keys().map(|p| p.clone()).collect::<Vec<String>>())
}

#[tauri::command(async)]
async fn categories(
    modules: State<'_, Modules2>,
    module_name: &str,
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
    module
        .categories
        .clone()
        .ok_or("Failed to fetch categories".to_string())
}

#[tauri::command(async)]
async fn ids(
    modules: State<'_, Modules2>,
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

#[tauri::command]
fn unsubscribe(listening: State<'_, Listening>, id: &str) -> () {
    let mut listening = listening.ids.lock().unwrap();

    if let Some(index) = listening.iter().position(|p| p.identifier == id) {
        listening.remove(index);
    };
}

#[tauri::command(async)]
async fn subscribe(
    listening: State<'_, Listening>,
    modules: State<'_, Modules2>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceConfig {
    pub addr: Arc<Mutex<Option<String>>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Listening {
    pub ids: Arc<Mutex<Vec<Function>>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Modules {
    pub modules: Vec<ModulePath>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Module2 {
    pub path: PathBuf,
    pub name: String,
    func: Option<HashMap<String, Function>>,
    categories: Option<Vec<String>>,
}

impl Module2 {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Modules2 {
    pub modules: Arc<Mutex<HashMap<String, Module2>>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModulePath {
    pub name: String,
    pub path: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(SourceConfig {
                addr: Arc::new(Mutex::new(None)),
            });
            app.manage(Listening {
                ids: Arc::new(Mutex::new(vec![])),
            });
            if let Some(user_profile) = env::var_os("USERPROFILE") {
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
                let dir = dcs_path.read_dir().unwrap();
                {
                    let map = dir
                        .map(|x| {
                            let path = x.unwrap().path();
                            ModulePath {
                                name: path
                                    .file_name()
                                    .unwrap()
                                    .to_os_string()
                                    .into_string()
                                    .unwrap(),
                                path: path,
                            }
                        })
                        .collect();
                    app.manage(Modules { modules: map });
                };
                let dir = dcs_path.read_dir().unwrap();
                let map = dir
                    .map(|x| {
                        let path = x.unwrap().path();
                        let name = path
                            .file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();
                        (
                            name.clone(),
                            Module2 {
                                name: name,
                                path: path,
                                categories: None,
                                func: None,
                            },
                        )
                    })
                    .collect::<HashMap<String, Module2>>();
                app.manage(Modules2 {
                    modules: Arc::new(Mutex::new(map)),
                });
            } else {
                eprintln!("USERPROFILE environment variable not found.");
                app.manage(Modules { modules: vec![] });
                app.manage(Modules2 {
                    modules: Arc::new(Mutex::new(HashMap::new())),
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            setup_socket,
            modules,
            categories,
            ids,
            subscribe,
            get_subscribed,
            unsubscribe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
