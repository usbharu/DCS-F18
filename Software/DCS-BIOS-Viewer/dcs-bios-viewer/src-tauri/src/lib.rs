mod data;
mod source;

use crate::data::Data;
use crate::source::{Source, UdpSource};
use dcs_bios::parse_packet;
use dcs_bios_const_generator::{parse_file, Function};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, iter, vec};
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
    let mut udp_source = UdpSource::from_addr(
        bind,
        &Ipv4Addr::from_str(addr).map_err(|_| "Addressが不正です".to_string())?,
        &Ipv4Addr::from_str(interface).map_err(|_| "Interfaceが不正です".to_string())?,
    ).map_err(|e| e.to_string())?;
    tauri::async_runtime::spawn({
        let source = source.addr.clone();
        let listening = listening.ids.clone();
        async move {
            let mut counter: u16 = 0;
            udp_source.setup().unwrap();
            let mut list: Vec<u16> = vec![];
            loop {
                if counter % 1000 == 0 {
                    let a = source.lock().unwrap();
                    let a = a.as_ref().unwrap();

                    list = listening
                        .lock()
                        .unwrap()
                        .iter()
                        .flat_map(|x| x.outputs.iter().map(|f| f.address).collect::<Vec<u16>>())
                        .collect::<Vec<u16>>();

                    if source_c != *a {
                        println!("break!");
                        break;
                    }
                }
                counter = counter.wrapping_add(1);

                match udp_source.read() {
                    Ok(v) => {
                        match parse_packet(&*v) {
                            None => {}
                            Some(v) => {
                                let mut map: Vec<Data> = v
                                    .iter()
                                    .filter_map(|x| {
                                        if x.address == 0 {
                                            return None;
                                        }
                                        if !list.contains(&x.address) {
                                            return None;
                                        }

                                        Some(Data {
                                            address: x.address,
                                            value: (u16::from_le_bytes(x.data) & 65535) >> 0,
                                        })
                                    })
                                    .collect::<Vec<Data>>();
                                if map.is_empty() {
                                    continue;
                                }

                                map.sort_by_key(|f| f.address);
                                map.dedup_by_key(|f| f.address);
                                app_handle.emit("data", map).unwrap();
                            }
                        };
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
fn modules(modules: State<'_, Modules>) -> Vec<String> {
    modules.modules.iter().map(|v| v.name.clone()).collect()
}

#[tauri::command]
fn categories(modules: State<'_, Modules>, module_name: &str) -> Result< Vec<String>,String> {
    let path = &modules
        .modules
        .iter()
        .find(|v| v.name == module_name)
        .unwrap()
        .path;
    let file = File::open(path).map_err(|e| e.to_string())?;
    let result: HashMap<String, HashMap<String, Function>> = match serde_json::from_reader(file) {
        Ok(v) => v,
        Err(_) => {
            return Ok(vec![]);
        }
    };
    Ok(result.iter().map(|v| v.0.clone()).collect())
}

#[tauri::command]
fn ids(modules: State<'_, Modules>, module_name: &str, category_name: &str) -> Vec<String> {
    let path = &modules
        .modules
        .iter()
        .find(|v| v.name == module_name)
        .unwrap()
        .path;
    let file = File::open(path).unwrap();
    parse_file(file)
        .unwrap()
        .iter()
        .filter(|p| p.category == category_name)
        .map(|v| v.identifier.clone())
        .collect()
}

#[tauri::command]
fn unsubscribe(listening: State<'_, Listening>, id: &str) -> () {
    let mut listening = listening.ids.lock().unwrap();

    if let Some(index) = listening.iter().position(|p| p.identifier == id) {
        listening.remove(index);
    };
}

#[tauri::command]
fn subscribe(
    listening: State<'_, Listening>,
    modules: State<'_, Modules>,
    module_name: &str,
    id: &str,
) -> () {
    let mut guard = listening.ids.lock().unwrap();
    let string = String::from(id);
    if guard.iter().find(|&c| *c.identifier == string).is_none() {
        let path = &modules
            .modules
            .iter()
            .find(|v| v.name == module_name)
            .unwrap()
            .path;
        let file = File::open(path).unwrap();
        match match parse_file(file) {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        }
        .iter()
        .find(|p| p.identifier == id)
        {
            Some(v) => {
                guard.push(v.clone());
                println!("subscribed {:?}", v);
            }
            None => {}
        }
    }
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

pub struct Modules {
    pub modules: Vec<ModulePath>,
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
            } else {
                eprintln!("USERPROFILE environment variable not found.");
                app.manage(Modules { modules: vec![] });
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
