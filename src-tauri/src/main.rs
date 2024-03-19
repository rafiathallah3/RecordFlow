// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(const_item_mutation)]

use std::str::FromStr;
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::string::ToString;
use enigo::{ Enigo, MouseControllable };
use tauri::{ CustomMenuItem, Manager, Menu, Submenu };
use tauri::App;
use serde::Serialize;
use rdev::{listen, Button, EventType, Key, simulate};

#[derive(Debug, Clone, Serialize)]
struct RawDataKey {
    tipe: String,
    value: String,
    waktu: f32
}

#[derive(Debug, Clone)]
struct DataKey {
    event_type: EventType,
    value: String,
    waktu: f32
}


#[derive(Debug)]
struct UpdateKey<'a>(&'a Key);
impl std::fmt::Display for UpdateKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl FromStr for UpdateKey<'_> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Alt" => Ok(UpdateKey(&Key::Alt)),
            "AltGr" => Ok(UpdateKey(&Key::AltGr)),
            "Backspace" => Ok(UpdateKey(&Key::Backspace)),
            "CapsLock" => Ok(UpdateKey(&Key::CapsLock)),
            "ControlLeft" => Ok(UpdateKey(&Key::ControlLeft)),
            "ControlRight" => Ok(UpdateKey(&Key::ControlRight)),
            "Delete" => Ok(UpdateKey(&Key::Delete)),
            "DownArrow" => Ok(UpdateKey(&Key::DownArrow)),
            "End" => Ok(UpdateKey(&Key::End)),
            "Escape" => Ok(UpdateKey(&Key::Escape)),
            "F1" => Ok(UpdateKey(&Key::F1)),
            "F10" => Ok(UpdateKey(&Key::F10)),
            "F11" => Ok(UpdateKey(&Key::F11)),
            "F12" => Ok(UpdateKey(&Key::F12)),
            "F2" => Ok(UpdateKey(&Key::F2)),
            "F3" => Ok(UpdateKey(&Key::F3)),
            "F4" => Ok(UpdateKey(&Key::F4)),
            "F5" => Ok(UpdateKey(&Key::F5)),
            "F6" => Ok(UpdateKey(&Key::F6)),
            "F7" => Ok(UpdateKey(&Key::F7)),
            "F8" => Ok(UpdateKey(&Key::F8)),
            "F9" => Ok(UpdateKey(&Key::F9)),
            "Home" => Ok(UpdateKey(&Key::Home)),
            "LeftArrow" => Ok(UpdateKey(&Key::LeftArrow)),
            "MetaLeft" => Ok(UpdateKey(&Key::MetaLeft)),
            "MetaRight" => Ok(UpdateKey(&Key::MetaRight)),
            "PageDown" => Ok(UpdateKey(&Key::PageDown)),
            "PageUp" => Ok(UpdateKey(&Key::PageUp)),
            "Return" => Ok(UpdateKey(&Key::Return)),
            "RightArrow" => Ok(UpdateKey(&Key::RightArrow)),
            "ShiftLeft" => Ok(UpdateKey(&Key::ShiftLeft)),
            "ShiftRight" => Ok(UpdateKey(&Key::ShiftRight)),
            "Space" => Ok(UpdateKey(&Key::Space)),
            "Tab" => Ok(UpdateKey(&Key::Tab)),
            "UpArrow" => Ok(UpdateKey(&Key::UpArrow)),
            "PrintScreen" => Ok(UpdateKey(&Key::PrintScreen)),
            "ScrollLock" => Ok(UpdateKey(&Key::ScrollLock)),
            "Pause" => Ok(UpdateKey(&Key::Pause)),
            "NumLock" => Ok(UpdateKey(&Key::NumLock)),
            "BackQuote" => Ok(UpdateKey(&Key::BackQuote)),
            "Num1" => Ok(UpdateKey(&Key::Num1)),
            "Num2" => Ok(UpdateKey(&Key::Num2)),
            "Num3" => Ok(UpdateKey(&Key::Num3)),
            "Num4" => Ok(UpdateKey(&Key::Num4)),
            "Num5" => Ok(UpdateKey(&Key::Num5)),
            "Num6" => Ok(UpdateKey(&Key::Num6)),
            "Num7" => Ok(UpdateKey(&Key::Num7)),
            "Num8" => Ok(UpdateKey(&Key::Num8)),
            "Num9" => Ok(UpdateKey(&Key::Num9)),
            "Num0" => Ok(UpdateKey(&Key::Num0)),
            "Minus" => Ok(UpdateKey(&Key::Minus)),
            "Equal" => Ok(UpdateKey(&Key::Equal)),
            "KeyQ" => Ok(UpdateKey(&Key::KeyQ)),
            "KeyW" => Ok(UpdateKey(&Key::KeyW)),
            "KeyE" => Ok(UpdateKey(&Key::KeyE)),
            "KeyR" => Ok(UpdateKey(&Key::KeyR)),
            "KeyT" => Ok(UpdateKey(&Key::KeyT)),
            "KeyY" => Ok(UpdateKey(&Key::KeyY)),
            "KeyU" => Ok(UpdateKey(&Key::KeyU)),
            "KeyI" => Ok(UpdateKey(&Key::KeyI)),
            "KeyO" => Ok(UpdateKey(&Key::KeyO)),
            "KeyP" => Ok(UpdateKey(&Key::KeyP)),
            "LeftBracket" => Ok(UpdateKey(&Key::LeftBracket)),
            "RightBracket" => Ok(UpdateKey(&Key::RightBracket)),
            "KeyA" => Ok(UpdateKey(&Key::KeyA)),
            "KeyS" => Ok(UpdateKey(&Key::KeyS)),
            "KeyD" => Ok(UpdateKey(&Key::KeyD)),
            "KeyF" => Ok(UpdateKey(&Key::KeyF)),
            "KeyG" => Ok(UpdateKey(&Key::KeyG)),
            "KeyH" => Ok(UpdateKey(&Key::KeyH)),
            "KeyJ" => Ok(UpdateKey(&Key::KeyJ)),
            "KeyK" => Ok(UpdateKey(&Key::KeyK)),
            "KeyL" => Ok(UpdateKey(&Key::KeyL)),
            "SemiColon" => Ok(UpdateKey(&Key::SemiColon)),
            "Quote" => Ok(UpdateKey(&Key::Quote)),
            "BackSlash" => Ok(UpdateKey(&Key::BackSlash)),
            "IntlBackslash" => Ok(UpdateKey(&Key::IntlBackslash)),
            "KeyZ" => Ok(UpdateKey(&Key::KeyZ)),
            "KeyX" => Ok(UpdateKey(&Key::KeyX)),
            "KeyC" => Ok(UpdateKey(&Key::KeyC)),
            "KeyV" => Ok(UpdateKey(&Key::KeyV)),
            "KeyB" => Ok(UpdateKey(&Key::KeyB)),
            "KeyN" => Ok(UpdateKey(&Key::KeyN)),
            "KeyM" => Ok(UpdateKey(&Key::KeyM)),
            "Comma" => Ok(UpdateKey(&Key::Comma)),
            "Dot" => Ok(UpdateKey(&Key::Dot)),
            "Slash" => Ok(UpdateKey(&Key::Slash)),
            "Insert" => Ok(UpdateKey(&Key::Insert)),
            "KpReturn" => Ok(UpdateKey(&Key::KpReturn)),
            "KpMinus" => Ok(UpdateKey(&Key::KpMinus)),
            "KpPlus" => Ok(UpdateKey(&Key::KpPlus)),
            "KpMultiply" => Ok(UpdateKey(&Key::KpMultiply)),
            "KpDivide" => Ok(UpdateKey(&Key::KpDivide)),
            "Kp0" => Ok(UpdateKey(&Key::Kp0)),
            "Kp1" => Ok(UpdateKey(&Key::Kp1)),
            "Kp2" => Ok(UpdateKey(&Key::Kp2)),
            "Kp3" => Ok(UpdateKey(&Key::Kp3)),
            "Kp4" => Ok(UpdateKey(&Key::Kp4)),
            "Kp5" => Ok(UpdateKey(&Key::Kp5)),
            "Kp6" => Ok(UpdateKey(&Key::Kp6)),
            "Kp7" => Ok(UpdateKey(&Key::Kp7)),
            "Kp8" => Ok(UpdateKey(&Key::Kp8)),
            "Kp9" => Ok(UpdateKey(&Key::Kp9)),
            "KpDelete" => Ok(UpdateKey(&Key::KpDelete)),
            "Function" => Ok(UpdateKey(&Key::Function)),
            _ => {
                if s.contains("Unknown") {
                    let hasil = s.replace("Unknown(", "").replace(")", "").parse::<u32>();
                    match hasil {
                        Ok(_n) => {
                            // Akan diperbaiki nanti
                            // let n_clone = &Clone::clone(&n);
                            return Ok(UpdateKey(&Key::Unknown(1)));
                        },
                        Err(..) => {
                            return Err("Unknown Key Value");
                        }
                    }
                }

                Err("Key is not valid!")
            }
        }
    }
}

struct UpdateButton<'a>(&'a Button);
impl std::fmt::Display for UpdateButton<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Button::Left => write!(f, "Left"),
            Button::Middle => write!(f, "Middle"),
            Button::Right => write!(f, "Right"),
            Button::Unknown(t) => write!(f, "{}", format!("Button: {}", t)),
        }
    }
}
impl FromStr for UpdateButton<'_> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" => Ok(UpdateButton(&Button::Left)),
            "Right" => Ok(UpdateButton(&Button::Right)),
            "Middle" => Ok(UpdateButton(&Button::Middle)),
            _ => {
                // Akan diperbaiki nanti
                Ok(UpdateButton(&Button::Unknown(1)))
            },
        }
    }
}

struct UpdateEventType(EventType);
impl std::fmt::Display for UpdateEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            EventType::ButtonPress(button) => write!(f, "{}", format!("Button Press {}", UpdateButton(button))),
            EventType::ButtonRelease(button) => write!(f, "{}", format!("Button Release {}", UpdateButton(button))),
            EventType::KeyPress(key) => write!(f, "{}", format!("Key Press {}", UpdateKey(key))),
            EventType::KeyRelease(key) => write!(f, "{}", format!("Key Release {}", UpdateKey(key))),
            EventType::MouseMove { x, y } => write!(f, "Mouse Move {} {}", x, y),
            EventType::Wheel { .. } => write!(f, "Mouse Wheel")
        }
    }
}

static mut DATA_MACRO: once_cell::sync::Lazy<Arc<Mutex<Vec<DataKey>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
static mut APAKAH_RECORD: Option<bool> = Some(false);
static mut APAKAH_LAGI_MAIN: Option<bool> = Some(false);
static mut WAKTU_MULAI_RECORD: once_cell::sync::Lazy<SystemTime> = once_cell::sync::Lazy::new(|| SystemTime::now());

fn ganti_state_recorder(apakah_record: &mut bool, window: &tauri::Window) {
    *apakah_record = !*apakah_record;
    unsafe {
        APAKAH_RECORD = Some(*apakah_record);
        if *apakah_record {
            println!("MUJLAI RECORD");
            DATA_MACRO.lock().unwrap().clear();
            WAKTU_MULAI_RECORD = once_cell::sync::Lazy::new(|| SystemTime::now());
        } else {
            // let data_macro = DATA_MACRO.lock().unwrap();

            // for val in data_macro.iter() {
            //     println!("{} {} {}", val.tipe, val.value, val.waktu);
            // }
        }
    }

    window.emit_all("StatusRecorder", *apakah_record).unwrap();
}

// sumber: https://docs.rs/rdev/latest/rdev/
fn send(event_type: &EventType) {
    // let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(..) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
}

#[tauri::command]
fn simpan_file(path: String) {
    let mut data_macro_clone: Vec<DataKey> = Vec::new();
    unsafe {
        let data_macro = DATA_MACRO.lock().unwrap();
        for val in data_macro.iter() {
            data_macro_clone.push(val.clone());
        }
    }

    let mut content_string = String::from("");
    for val in data_macro_clone {
        let mut value_event = String::from("");

        if matches!(val.event_type, EventType::ButtonPress(..)) || matches!(val.event_type, EventType::ButtonRelease(..)) || matches!(val.event_type, EventType::MouseMove {..}) || matches!(val.event_type, EventType::Wheel {..}) {
            value_event = val.value;
        }

        if let EventType::KeyPress(key) = val.event_type {
            value_event = UpdateKey(&key).to_string();
        }
        if let EventType::KeyRelease(key) = val.event_type {
            value_event = UpdateKey(&key).to_string();
        }

        let string_val = format!("{}|||{}|||{}\n", UpdateEventType(val.event_type).to_string(), value_event, val.waktu);
        content_string.push_str(&string_val);
    }

    fs::write(path, content_string).unwrap();
}

#[tauri::command]
fn buka_file(window: tauri::Window, path: String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    unsafe {
        let mut data_macro = DATA_MACRO.lock().unwrap();
        data_macro.clear();

        for val in contents.split("\n") {
            let raw_data_macro: Vec<&str> = val.split("|||").collect();
            if raw_data_macro.len() <= 0 {
                continue
            }

            if raw_data_macro[0].contains("Key Press") || raw_data_macro[0].contains("Key Release") {
                let raw_key = UpdateKey::from_str(raw_data_macro[1]);
                match raw_key {
                    Ok(k) => {
                        match raw_data_macro[2].parse::<f32>() {
                            Ok(n) => {
                                let event_type: EventType;
                                if raw_data_macro[0].contains("Key Press") {
                                    event_type = EventType::KeyPress(*k.0);
                                } else {
                                    event_type = EventType::KeyRelease(*k.0);
                                }

                                data_macro.push(DataKey { event_type, value: k.to_string(), waktu: n })
                            },
                            Err(..) => continue
                        }
                    },
                    Err(..) => continue
                }
            }

            if raw_data_macro[0].contains("Button Press") || raw_data_macro[0].contains("Button Release") || raw_data_macro[0].contains("Mouse Wheel") {
                let split_str_0: Vec<&str> = raw_data_macro[0].split(" ").collect();
                let mut tombol_ditekan = String::from("");
                
                if raw_data_macro[0].contains("Button Press") || raw_data_macro[0].contains("Button Release") {
                    if split_str_0.len() <= 0 || split_str_0.len() > 3 {
                        continue
                    }

                    tombol_ditekan = split_str_0[2].to_string();     
                }
                
                if raw_data_macro[0].contains("Mouse Wheel") {
                    tombol_ditekan = String::from("Wheel");
                }

                let raw_button = UpdateButton::from_str(&tombol_ditekan);
                match raw_button {
                    Ok(b) => {
                        match raw_data_macro[2].parse::<f32>() {
                            Ok(n) => {
                                let event_type: EventType;
                                if raw_data_macro[0].contains("Button Press") {
                                    event_type = EventType::ButtonPress(*b.0);
                                } else if raw_data_macro[0].contains("Button Release") {
                                    event_type = EventType::ButtonRelease(*b.0);
                                } else {
                                    let raw_split_value: Vec<&str> = raw_data_macro[1].split(", ").collect();
                                    let value_0 = match raw_split_value[0].parse::<i64>() {
                                        Ok(n) => n,
                                        Err(..) => continue
                                    };
                                    let value_1 = match raw_split_value[1].parse::<i64>() {
                                        Ok(n) => n,
                                        Err(..) => continue
                                    };

                                    event_type = EventType::Wheel { delta_x: value_0, delta_y: value_1 };
                                }

                                data_macro.push(DataKey { event_type, value: raw_data_macro[1].to_string(), waktu: n })
                            }
                            Err(..) => continue
                        }
                    },
                    Err(..) => continue
                }
            }
        }

        for val in data_macro.iter() {
            let data_key = RawDataKey { tipe: UpdateEventType(val.event_type).to_string(), value: val.value.clone(), waktu: val.waktu.clone() };
            window.emit_all("KirimDataInput", &data_key).unwrap();
            // println!("{:?} {} {}", val.event_type, val.value, val.waktu);
        }
    }
    
}

#[tauri::command]
fn mulai_record(window: tauri::Window) {
    unsafe {
        if let Some(apakah_lagi_main) = APAKAH_LAGI_MAIN {
            if apakah_lagi_main {
                return
            }
        }

        if let Some(mut apakah_record) = APAKAH_RECORD {
            ganti_state_recorder(&mut apakah_record, &window);
        }
    }
}

#[tauri::command]
async fn mainkan_recorder(window: tauri::Window) {
    unsafe {
        if let Some(apakah_record) = APAKAH_RECORD {
            if apakah_record {
                return
            }
        }

        if let Some(apakah_lagi_main) = APAKAH_LAGI_MAIN {
            if apakah_lagi_main {
                return
            }
        }

        APAKAH_LAGI_MAIN = Some(true);
    }

    let mut data_macro_clone: Vec<DataKey> = Vec::new();
    unsafe {
        let data_macro = DATA_MACRO.lock().unwrap();
        for val in data_macro.iter() {
            //Kenapa tidak sekalian mainkan recordernya disini? Tidak tau kenapa alasannya tetapi ketika melakukan thread::sleep disini akan menimbulkan Lag bagi cursor.
            data_macro_clone.push(val.clone());
        }
    }

    if data_macro_clone.len() <= 0 {
        unsafe {
            APAKAH_LAGI_MAIN = Some(false);
        }
        return;
    }

    let waktu_di_tunggu = SystemTime::now();
    for val in data_macro_clone.iter() {
        let mut waktu_sekarang_tunggu = SystemTime::now();
        while waktu_sekarang_tunggu.duration_since(waktu_di_tunggu).unwrap().as_secs_f32() < val.waktu {
            waktu_sekarang_tunggu = SystemTime::now();
        }

        if matches!(val.event_type, EventType::ButtonPress(..))|| matches!(val.event_type, EventType::ButtonRelease(..)) {
            let posisi_mos: Vec<&str> = val.value.split(", ").collect();
            send(&EventType::MouseMove { x: posisi_mos[0].parse::<f64>().unwrap(), y: posisi_mos[1].parse::<f64>().unwrap() });
        }

        send(&val.event_type);
    }

    window.emit_all("SelesaiRecording", true).unwrap();

    thread::sleep(Duration::from_millis(500));
    unsafe {   
        APAKAH_LAGI_MAIN = Some(false);
    }
}

fn create_app_menu() -> Menu {
    return Menu::new()
        .add_submenu(Submenu::new("File", Menu::new()
            .add_item(CustomMenuItem::new("simpan".to_string(), "Save"))
            .add_item(CustomMenuItem::new("buka".to_string(), "Load"))
        ));
}

fn setup(app: &mut App) -> Result<(), Box<(dyn std::error::Error + 'static)>> { // Not entirely sure, but perhaps you could omit that error type  
    let main_window = app.get_window("main").unwrap().clone();
    let window_mutex = Arc::new(Mutex::new(main_window));
    let data_macro_murtex = unsafe { DATA_MACRO.clone() };

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            let enigo = Enigo::new();
            let window = window_mutex.lock().unwrap();
            let mut data_macro = data_macro_murtex.lock().unwrap();
            //Idea: Memberi opsi untuk me-record mouse movement dan mengubah value button press/release menjadi posisi mouse
            
            unsafe {
                if let EventType::KeyRelease(key) = event.event_type {
                    if key == Key::F6 {
                        if let Some(apakah_lagi_main) = APAKAH_LAGI_MAIN {
                            if apakah_lagi_main {
                                return
                            }
                        }

                        if let Some(mut apakah_record) = APAKAH_RECORD {
                            apakah_record = !apakah_record;
                            APAKAH_RECORD = Some(apakah_record);
                            if apakah_record {
                                data_macro.clear();
                                WAKTU_MULAI_RECORD = once_cell::sync::Lazy::new(|| SystemTime::now());
                            } else {
                                // for val in data_macro.iter() {
                                //     println!("{} {} {}", val.tipe, val.value, val.waktu);
                                // }
                            }

                            window.emit_all("StatusRecorder", &apakah_record).unwrap();
                        }
                    } else if key == Key::F7  {
                        println!("TEKAN F7!");
                        //KITA HARUS STOP PLAYING KETIKA USER KLIK LAGI
                        if let Some(apakah_record) = APAKAH_RECORD {
                            if apakah_record {
                                return
                            }
                        }
                
                        if let Some(apakah_lagi_main) = APAKAH_LAGI_MAIN {
                            if apakah_lagi_main {
                                APAKAH_LAGI_MAIN = Some(false);
                                return
                            }
                        }
                
                        if data_macro.len() <= 0 {
                            return;
                        }

                        let data_macro_clone = data_macro.clone();
                        let window_clone = window.clone();
                        thread::spawn(move || {
                            APAKAH_LAGI_MAIN = Some(true);
                            
                            let waktu_di_tunggu = SystemTime::now();
                            for val in data_macro_clone.iter() {
                                if let Some(apakah_lagi_main) = APAKAH_LAGI_MAIN {
                                    if !apakah_lagi_main {
                                        break
                                    }
                                }

                                let mut waktu_sekarang_tunggu = SystemTime::now();
                                while waktu_sekarang_tunggu.duration_since(waktu_di_tunggu).unwrap().as_secs_f32() < val.waktu {
                                    waktu_sekarang_tunggu = SystemTime::now();
                                }

                                if matches!(val.event_type, EventType::ButtonPress(..))|| matches!(val.event_type, EventType::ButtonRelease(..)) {
                                    let posisi_mos: Vec<&str> = val.value.split(", ").collect();
                                    send(&EventType::MouseMove { x: posisi_mos[0].parse::<f64>().unwrap(), y: posisi_mos[1].parse::<f64>().unwrap() });
                                }

                                send(&val.event_type);
                            }

                            window_clone.emit_all("SelesaiRecording", true).unwrap();

                            thread::sleep(Duration::from_millis(500));
                            APAKAH_LAGI_MAIN = Some(false);
                        });
                    } else {
                        if let Some(apakah_record) = APAKAH_RECORD {
                            if !apakah_record {
                                return;
                            }

                            let waktu_sekarang = SystemTime::now();
                            let selisih_waktu = match waktu_sekarang.duration_since(*WAKTU_MULAI_RECORD) {
                                Ok(durasi) =>  durasi.as_secs_f32(),
                                Err(..) => 0.0
                            };

                            let data_key = RawDataKey { tipe: UpdateEventType(event.event_type).to_string(), value: UpdateKey(&key).to_string(), waktu: selisih_waktu };
                            window.emit_all("KirimDataInput", &data_key).unwrap();
                            data_macro.push(DataKey { event_type: event.event_type, waktu: selisih_waktu, value: UpdateKey(&key).to_string() });
                        }
                    }
                }

                if let Some(apakah_record) = APAKAH_RECORD {
                    if matches!(event.event_type, EventType::MouseMove { .. }) || !apakah_record {
                       return; 
                    }

                    let waktu_sekarang = SystemTime::now();
                    let selisih_waktu = match waktu_sekarang.duration_since(*WAKTU_MULAI_RECORD) {
                        Ok(durasi) =>  durasi.as_secs_f32(),
                        Err(..) => 0.0
                    };

                    if matches!(event.event_type, EventType::ButtonPress(..)) || matches!(event.event_type, EventType::ButtonRelease(..)) {
                        let posisi_mos = enigo.mouse_location();
                        let data_key = RawDataKey { tipe: UpdateEventType(event.event_type).to_string(), value: format!("{}, {}", posisi_mos.0, posisi_mos.1), waktu: selisih_waktu };
                        window.emit_all("KirimDataInput", &data_key).unwrap();
                        data_macro.push(DataKey { event_type: event.event_type, waktu: selisih_waktu, value: format!("{}, {}", posisi_mos.0, posisi_mos.1) });
                    }

                    if let EventType::Wheel { delta_x, delta_y } = event.event_type {
                        let data_key = RawDataKey { tipe: UpdateEventType(event.event_type).to_string(), value: format!("{}, {}", delta_x, delta_y), waktu: selisih_waktu };
                        window.emit_all("KirimDataInput", &data_key).unwrap();
                        data_macro.push(DataKey { event_type: event.event_type, waktu: selisih_waktu, value: format!("{}, {}", delta_x, delta_y) });
                    }
    
                    if let EventType::KeyPress(key) = event.event_type {
                        let data_key = RawDataKey { tipe: UpdateEventType(event.event_type).to_string(), value: UpdateKey(&key).to_string(), waktu: selisih_waktu };
                        window.emit_all("KirimDataInput", &data_key).unwrap();
                        data_macro.push(DataKey { event_type: event.event_type, waktu: selisih_waktu, value: UpdateKey(&key).to_string() });
                    }
                }
            }

        }) {
            println!("Error: {:?}", error)
        }
    });

    return Ok(());
}

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![mulai_record, mainkan_recorder, simpan_file, buka_file])
        .menu(create_app_menu())
        .on_menu_event(move | event | {
            println!("{subject}", subject=event.menu_item_id());
            match event.menu_item_id() {
                "simpan" => {
                    event.window().emit_all("DapatinSimpananFile", true).unwrap();
                }
                "buka" => {
                    event.window().emit_all("BukaFile", true).unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    event.window().close().unwrap();
                }
                _ => {}
            }
        })
        // .setup(|app| {
        //     return setup(app);
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}