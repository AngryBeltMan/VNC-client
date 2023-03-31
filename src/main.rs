use image::{codecs::jpeg::JpegEncoder, ColorType};
use async_channel;
use crate::key::ToKey;
use rdev::{simulate,Button,EventType,Key,SimulateError};
use lazy_static::lazy_static;
use async_mutex::Mutex;
use std::{sync::Mutex as StdMutex};
use tungstenite::Message;
use std::{
    time::Duration
    ,thread::{sleep, self}
    ,sync::Arc
};
use tokio::{sync::mpsc,net::TcpStream, runtime::Runtime};
use scrap::{Capturer,Display};
use futures_util::{stream::{SplitStream, SplitSink}, SinkExt, StreamExt};
use tokio_tungstenite::*;
use eframe::egui;

extern crate reqwest;

mod gui;
mod websocket;
mod key;
mod random_code;

const FRAMEDECODERS:u32 = 1;
const DELAY:Duration = Duration::from_millis(60);
const URL:&str = "vnc-shuttle.shuttleapp.rs";
lazy_static! {
    static ref EGUI_DELAY:Arc<StdMutex<Duration>> = Arc::new(StdMutex::new(Duration::from_millis(60)));
    static ref QUALITY:Arc<StdMutex<u32>> = Arc::new(StdMutex::new(0));
    static ref RUN_TIME:Runtime = {
        tokio::runtime::Runtime::new().unwrap()
    };
    static ref JOIN_CODE:String = {
        random_code::generate_code(5)
    };
}
async fn send(event:&EventType) {
    match  simulate(event) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event);
        }
    }
}
#[tokio::main(flavor = "multi_thread",worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_ne!(FRAMEDECODERS,0);
    let url = format!("https://{URL}/server/{}",JOIN_CODE.as_str());
    reqwest::Client::new().post(url).send().await?;
    let url = format!("wss://{URL}/ws/client/keyboard/{}",JOIN_CODE.as_str());
    let mut stream = connect_async(url).await.unwrap().0;
    let url = format!("wss://{URL}/ws/{}",JOIN_CODE.as_str());
    let conn = connect_async(url).await?.0;
    let mut sink = conn.split().0;
    let d = Display::primary().unwrap();
    let w = d.width();
    thread::spawn(move || {
        RUN_TIME.block_on(async move {
            println!("connected");
            while let Some(o) = stream.next().await {
                if let Ok(o) = o {
                    println!("found command");
                    match o {
                        Message::Text(string) => {
                            let move_ment = string.split(",").collect::<Vec<&str>>();
                            match move_ment[0] {
                                "MOUSEMOVE" => {
                                    let x = move_ment[1].parse::<f64>().unwrap() * (w as f64/move_ment[3].parse::<f64>().unwrap());
                                    let y = move_ment[2].parse::<f64>().unwrap() * (w as f64/move_ment[4].parse::<f64>().unwrap());
                                    send(&EventType::MouseMove {x,y}).await;
                                    println!("moved {x} {y}");
                                },
                                "KEYDOWN" => {
                                    if let Some(key) = move_ment[1].to_lowercase().to_key() {
                                        send(&EventType::KeyPress(key)).await;
                                    }
                                    println!("key down!");
                                },
                                "KEYUP" => {
                                    if let Some(key) = move_ment[1].to_lowercase().to_key() {
                                        send(&EventType::KeyRelease(key)).await;
                                    }
                                    println!("key up!");
                                },
                                "MOUSEDOWN" => {
                                    match move_ment[1] {
                                        "LEFT" => {
                                            send(&EventType::ButtonPress(Button::Left)).await;
                                        },
                                        "RIGHT" => {
                                            send(&EventType::ButtonPress(Button::Right)).await;
                                        },
                                        "MIDDLE" => {
                                            send(&EventType::ButtonPress(Button::Middle)).await;
                                        },
                                         _ => {},
                                    }
                                },
                                "MOUSEUP" => {
                                    match move_ment[1] {
                                        "LEFT" => {
                                            send(&EventType::ButtonRelease(Button::Left)).await;
                                        },
                                        "RIGHT" => {
                                            send(&EventType::ButtonRelease(Button::Right)).await;
                                        },
                                        "MIDDLE" => {
                                            send(&EventType::ButtonRelease(Button::Middle)).await;
                                        },
                                         _ => {},
                                    }
                                }
                                _ => {},
                            }
                        },
                        _ => {}
                    }
                }
            }
        });
    });
    thread::spawn(move || {
        let d = Display::primary().unwrap();
        let (w, h) = (d.width(), d.height());
        let mut capturer = Capturer::new(d).unwrap();
        loop {
            match capturer.frame() {
                Ok(frame) => {
                    let mut bitflipped = Vec::with_capacity(w * h  * 3);
                    let stride = frame.len() / h as usize;
                    for y in 0..h {
                        for x in 0..w {
                            let i = stride * y + 4 * x;
                            bitflipped.extend_from_slice(&[
                                                         frame[i + 2],
                                                         frame[i + 1],
                                                         frame[i],
                            ]);
                        }
                    }
                    let mut buffer:Vec<u8> = Vec::new();
                    let mut encoder = JpegEncoder::new_with_quality(&mut buffer,10);
                    encoder.encode(&bitflipped, w as u32, h as u32, ColorType::Rgb8).unwrap();
                    println!("encoded");
                    RUN_TIME.block_on(async {
                        sink.send(Message::Binary(buffer)).await.unwrap();
                    });
                    println!("sent");
                    // if let Ok(d) = EGUI_DELAY.lock() { sleep(*d); } else { sleep(DELAY); }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Wait for the frame.
                    continue;
                }
                Err(err) => {
                    println!("error in frames production {:?}",err);
                }
            }
        } }); let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc|
            Box::new(
                gui::MyApp::new(&*JOIN_CODE)
            )),
    )?;
    let url = format!("https://{URL}/server/{}",JOIN_CODE.as_str());
    reqwest::Client::new().post(url).send().await?;
    Ok(())
}
