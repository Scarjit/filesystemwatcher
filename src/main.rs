use std::collections::HashMap;
use config::ConfigError;
use std::path::Path;
use std::{thread, io};
use std::fs::File;
use std::io::{BufReader, Write, Read};
use rodio::{Source, Device, Decoder};
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event, EventKind};

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();



    if let Ok(highlightspath_str) = settings.get_str("path"){
        let record_path = Path::new(&highlightspath_str);
        if !record_path.exists(){
            println!("Invalid replay path !");
        }else {
            let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| {
                match res {
                    Ok(event) => play_sound(),
                    Err(e) => println!("watch error: {:?}", e),
                }
            }).unwrap();

            watcher.watch(highlightspath_str, RecursiveMode::Recursive).unwrap();

            loop {}
        }

    }else{
        println!("NVIDIA Highlights path missing");
    }
}

pub fn play_sound(){
    let device = rodio::default_output_device().unwrap();
    let source = rodio::Decoder::new(BufReader::new(File::open(&"sound.wav").unwrap())).unwrap();
    rodio::play_raw(&device, source.convert_samples());

    thread::sleep(Duration::from_secs(5));
}
