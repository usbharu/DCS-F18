use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}, time::Duration};
use tauri::{Emitter, Manager, State};
use tokio::time;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(source: State<'_, Source>, name: &str) -> String {
    println!("{:?}", source.addr.lock().unwrap());
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn setup_socket(
    source: State<'_, Source>,
    app_handle: tauri::AppHandle,
    bind: &str,
    addr: &str,
    interface: &str,
) -> Result<(), String> {

    println!("bind: {} addr: {} interface: {}", bind, addr, interface);
    
    *(source.addr.lock().unwrap()) = Some(bind.to_string());

    let source_c = bind.to_string();    
    tauri::async_runtime::spawn({
        let source = source.addr.clone();
        async move {
            loop {
                {
                    let a = source.lock().unwrap();
                    let a=  a.as_ref().unwrap();
                    if source_c != *a {
                        break;
                    }
                    
                    
                    app_handle.emit("source",a).unwrap();
                }

                time::sleep(Duration::from_secs(1)).await;
            }
            
        }
    });
    Ok(())
}

#[tauri::command]
fn category() -> Vec<String> {
    vec!["FA-18C".to_string(), "Common".to_string()]
}

#[tauri::command]
fn ids(category_name: &str) -> Vec<String> {
    vec!["Altitude".to_string()]
}

#[tauri::command]
fn subscribe(listening: State<'_, Listening>, id: &str) -> () {
    let mut guard = listening.ids.lock().unwrap();
    let string = String::from(id);
    if guard.iter().find(|&c| *c == string).is_none() {
        guard.push(string);
    }
}

#[tauri::command]
fn get_subscribed(listening: State<'_, Listening>) -> Vec<String> {
    let guard = listening.ids.lock().unwrap();
    guard.clone()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub addr: Arc<Mutex<Option<String>>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Listening {
    pub ids: Mutex<Vec<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Source {
                addr: Arc::new(Mutex::new(None)),
            });
            app.manage(Listening {
                ids: Mutex::new(vec![]),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            setup_socket,
            category,
            ids,
            subscribe,
            get_subscribed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
