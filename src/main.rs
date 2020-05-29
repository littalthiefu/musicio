use crossbeam_channel::bounded;
use directories::BaseDirs;
use libpulse_binding::error::Code;
use libpulse_binding::sample;
use libpulse_binding::stream::Direction;
use libpulse_simple_binding::Simple;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::thread;

#[derive(Clone, Deserialize, Serialize)]
struct Config {
    sources: Vec<Source>,
    output: Output,
}

#[derive(Clone, Deserialize, Serialize)]
struct Source {
    name: String,
    description: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct Output {
    name: String,
}

fn main() {
    // Load config file
    let path = BaseDirs::new()
        .unwrap()
        .config_dir()
        .join(PathBuf::from("musicio"));

    match fs::create_dir(&path) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {}
            _ => {
                println!("error creating dir {}: {}", path.to_str().unwrap(), e);
                return;
            }
        },
    }

    let path = path.join(PathBuf::from("config"));
    let config = match fs::read_to_string(&path) {
        Ok(n) => match toml::from_str::<Config>(&n) {
            Ok(n) => n,
            Err(e) => {
                println!("Error parsing config file: {}", e);
                return;
            }
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                // Write default config file
                let config = Config {
                    sources: vec![
                        Source {
                            name: "alsa_output.pci-0000_00_1f.3.analog-stereo.monitor".to_string(),
                            description: "Monitor of Output".to_string(),
                        },
                        Source {
                            name: "alsa_input.pci-0000_00_1f.3.analog-stereo".to_string(),
                            description: "Input".to_string(),
                        },
                    ],
                    output: Output {
                        name: "null".to_string(),
                    },
                };

                match fs::write(&path, toml::to_string_pretty(&config).unwrap()) {
                    Ok(_) => {}
                    Err(e) => println!(
                        "failed to write new config file ({}): {}",
                        path.to_str().unwrap(),
                        e
                    ),
                };

                config
            }
            _ => {
                println!("error reading {}: {}", path.to_str().unwrap(), e);
                return;
            }
        },
    };

    let spec = sample::Spec {
        format: sample::SAMPLE_S32NE,
        channels: 2,
        rate: 44100,
    };

    let (tx, rx) = bounded(1);

    for source in config.clone().sources.into_iter() {
        println!("Recording from '{}'", source.description);
        let tx = tx.clone();

        thread::spawn(move || {
            let simple = match Simple::new(
                None,
                "MusicIO",
                Direction::Record,
                Some(&source.name),
                &source.description,
                &spec,
                None,
                None,
            ) {
                Ok(n) => n,
                Err(e) => {
                    match e.into() {
                        Code::NoEntity => println!("NoEntity ({}) returned by PulseAudio - are you sure the source '{}' exists?", e.0, source.name),
                        n => println!("{:?} ({}) returned by PulseAudio - error connecting to the server for source '{}'", n, e.0, source.name),
                    }
                    return;
                }
            };

            loop {
                let mut buf = [0u8; 32];
                simple.read(&mut buf).unwrap();

                tx.send(buf).unwrap();
            }
        });
    }

    println!("Playback to {}", config.output.name);

    let simple = match Simple::new(
        None,
        "MusicIO",
        Direction::Playback,
        Some(&config.output.name),
        &config.output.name,
        &spec,
        None,
        None,
    ) {
        Ok(n) => n,
        Err(e) => {
            match e.into() {
                Code::NoEntity => println!("NoEntity ({}) returned by PulseAudio - are you sure the sink '{}' exists?", e.0, config.output.name),
                n => println!("{:?} ({}) returned by PulseAudio - error connecting to the server for sink '{}'", n, e.0, config.output.name),
            }
            return;
        }
    };

    while let Ok(buf) = rx.recv() {
        match simple.write(&buf) {
            Ok(_) => {}
            Err(e) => {
                let n: Code = e.into();
                println!("{:?} ({}) returned by PulseAudio - error connecting to the server for null sink", n, e.0)
            }
        };
    }
}
