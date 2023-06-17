// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

// use std::
use std::sync::Arc;
use std::sync::Mutex;

use serialport;
use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::thread;
use std::time::{Duration, Instant};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_fft_result(rx: tauri::State<Arc<Mutex<Vec<Vec<f32>>>>>) -> Vec<Vec<f32>> {
    let guard = rx.lock().unwrap();

    (*guard).clone()
}
fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
    // let (tx, rx) = channel();
    let fft_result: Arc<Mutex<Vec<Vec<f32>>>> = Arc::new(Mutex::new(vec![
        Vec::with_capacity(128),
        Vec::with_capacity(128),
        Vec::with_capacity(128),
    ]));
    let fft_result_s = Arc::clone(&fft_result);
    thread::spawn(move || {
        let package_len = 7808;
        let mut serial_buf: Vec<u8> = vec![0; package_len];
        loop {
            let start = Instant::now();
            port.read_exact(serial_buf.as_mut_slice()).unwrap();
            let elasped = start.elapsed();
            let sample_rate = (128.0 / elasped.as_secs_f32()) as u32;
            println!("Sample Rate:: {sample_rate} Hz");
            let mut index_mark = 0;
            let mut buffered_lines = vec![];
            serial_buf.iter().for_each(|item| {
                if *item == '\n' as u8 {
                    if index_mark <= 70 {
                        index_mark += 1;
                        return;
                    }
                    let end = index_mark + 61;
                    if end > package_len - 1 {
                        return;
                    }
                    buffered_lines.push(&serial_buf[(index_mark + 1)..end])
                }
                index_mark += 1;
            });

            let mut fft_batch = vec![
                Vec::with_capacity(127),
                Vec::with_capacity(127),
                Vec::with_capacity(127),
                Vec::with_capacity(127),
            ];
            buffered_lines.iter().for_each(|item| {
                let line = String::from_utf8_lossy(item);
                // println!("{line}");
                // if line.trim().len() != 60 {
                //     return;
                // }
                // print!("{}",line.len());
                let xa: f32 = line[1..8].trim_start().parse().unwrap_or_else(|_err| {
                    // panic!("{line}");
                    0.0
                });
                let ya: f32 = line[9..16].trim_start().parse().unwrap_or_else(|_err| {
                    // panic!("{line}");
                    0.0
                });
                let za: f32 = line[17..24].trim_start().parse().unwrap_or_else(|_err| {
                    // panic!("{line}");
                    0.0
                });
                fft_batch[0].push(xa);
                fft_batch[1].push(ya);
                fft_batch[2].push(za);
                fft_batch[3].push(f32::sqrt(xa * xa + ya * ya + za * za));
                // println!("{}",);
            });
            // fft_batch[0].resize(256, 0.0);
            // fft_batch[1].resize(256, 0.0);
            // fft_batch[2].resize(256, 0.0);
            fft_batch[0].resize(128, 0.0);
            fft_batch[1].resize(128, 0.0);
            fft_batch[2].resize(128, 0.0);
            fft_batch[3].resize(128, 0.0);
            let xyz_spec: Vec<Vec<f32>> = fft_batch
                .iter()
                .map(|xyz| {
                    // let mut index = 0;
                    // let test_sam :Vec<f32> = (0..128).map(move |item|{
                    //     // if index % 2 == 0{
                    //     //     index+=1;
                    //     //      vec![1.0]
                    //     // }
                    //     // else {
                    //     //     index+=1;
                    //     //    vec![-1.0]
                    //     // }
                    // }).flatten().collect();
                    // let test_sam = test_sam.iter().flatten();
                    // let hann_window = blackman_harris_4term(&xyz);
                    // calc spectrum
                    let spectrum_hann_window = samples_fft_to_spectrum(
                        // (windowed) samples
                        &xyz,
                        // sampling rate
                        sample_rate,
                        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
                        FrequencyLimit::All,
                        // optional scale
                        Some(&divide_by_N),
                        // None,
                    )
                    .unwrap();
                    let spec_vec: Vec<f32> = spectrum_hann_window
                        .data()
                        .iter()
                        .map(|(_f, v)| (*v).val())
                        .collect();
                    spec_vec
                })
                .collect();
            // println!("{}",xyz_spec[1][0]);
            let mut guard = fft_result.lock().unwrap();
            (*guard) = xyz_spec;
        }
    });
    tauri::Builder::default()
        //     .setup(|app| {
        //     // listen to the `event-name` (emitted on any window)
        //     // let id = app.listen_global("event-name", |event| {
        //     //   println!("got event-name with payload {:?}", event.payload());
        //     // });
        //     // // unlisten to the event using the `id` returned on the `listen_global` function
        //     // // a `once_global` API is also exposed on the `App` struct
        //     // app.unlisten(id);
        //     // // emit the `event-name` event to all webview windows on the frontend
        //     app.emit_all("event-name", 1).unwrap();
        //     Ok(())
        //   })
        .manage(fft_result_s)
        .invoke_handler(tauri::generate_handler![get_fft_result])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
