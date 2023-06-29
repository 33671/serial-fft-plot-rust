// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serialport;
use serialport::ClearBuffer;
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri_plot::{collect_fft, SerialParameter, PACKAGE_LEN};
static mut SAMPLE_RATE: AtomicU32 = AtomicU32::new(1440);
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_fft_result(vect: tauri::State<Arc<Mutex<Vec<Vec<f32>>>>>) -> Vec<Vec<f32>> {
    let guard = vect.lock().unwrap();
    (*guard).clone()
}
#[tauri::command]
fn get_sample_rate() -> u32 {
    unsafe { SAMPLE_RATE.load(Ordering::Relaxed) }
}
#[tauri::command]
fn list_ports() -> Vec<String> {
    let ports = serialport::available_ports().unwrap();
    ports.iter().map(|port| port.port_name.clone()).collect()
}

#[tauri::command]
fn listen_port(tx: tauri::State<Mutex<Sender<SerialParameter>>>, port: String, baud_rate: i32) {
    let guard = tx.lock().unwrap();
    guard
        .send(SerialParameter {
            port_name: port,
            baud_rate,
        })
        .unwrap();
}
fn main() {
    let fft_result: Arc<Mutex<Vec<Vec<f32>>>> = Arc::new(Mutex::new(vec![]));
    let fft_result_s = Arc::clone(&fft_result);
    let (tx, rx) = channel::<SerialParameter>();
    let tx_t = tx.clone();

    thread::spawn(move || loop {
        let port = rx.recv().unwrap();
        println!(
            "Port Selected:{},Baud Rate:{}",
            port.port_name, port.baud_rate
        );
        let mut port = serialport::new(port.port_name, port.baud_rate as u32)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        port.set_timeout(Duration::from_millis(2000))
            .unwrap_or_default();
        let mut buf = [0 as u8; PACKAGE_LEN];
        loop {
            port.clear(ClearBuffer::Input).unwrap_or_default();
            let start = Instant::now();
            let read_result = port.read_exact(&mut buf);
            if read_result.is_err() {
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            let elasped = start.elapsed();
            unsafe {
                *SAMPLE_RATE.get_mut() =
                    ((PACKAGE_LEN as f32 / 8.0) / elasped.as_secs_f32()) as u32;
            }
            let xyz_spec = collect_fft(&buf);
            {
                let mut guard = fft_result.lock().unwrap();
                (*guard) = xyz_spec;
            }

            let pending_rx = rx.try_recv();
            if pending_rx.is_ok() {
                tx_t.send(pending_rx.unwrap()).unwrap_or_default();
                break;
            }
        }
    });
    tauri::Builder::default()
        .manage(fft_result_s)
        .manage(Mutex::new(tx.clone()))
        .invoke_handler(tauri::generate_handler![
            get_fft_result,
            list_ports,
            listen_port,
            get_sample_rate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
