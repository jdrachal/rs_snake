use std::convert::TryInto;
use std::{thread};
use std::collections::{HashMap, LinkedList};
use serde::{Deserialize, Serialize};
use std::time::{Duration};

extern crate reqwest;
use std::io::{stdin, Read};
use simple_matrix::Matrix;
use matrix_display::*;
use console::Term;

use anyhow::Result;

extern crate signal_hook;

use crossbeam_channel::{bounded, tick, Receiver, select};
use ctrlc;

const MAP_WIDTH: usize = 16;

#[derive(Clone, Copy, Eq, Serialize, Deserialize, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct SnakeState {
    direction: String,
    positions: LinkedList<Position>
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct GameState {
    snake_state: SnakeState,
    fruit_positions: Vec<Position>,
    user_input: HashMap<String, u32>
}

async fn update_map() {
    let mut matrix: Matrix<char> = Matrix::new(16, 16);
    let res = reqwest::get("http://127.0.0.1:8080/snake").await.unwrap();
    let body = res.text().await.unwrap();

    let game_state: GameState =  serde_json::from_str(body.as_str()).unwrap();

    for f in game_state.fruit_positions.clone() {
        matrix.set(f.y.try_into().unwrap(), f.x.try_into().unwrap(), 'x');
    }

    for p in game_state.snake_state.positions.clone() {
        if p == *game_state.snake_state.positions.back().unwrap() {
            matrix.set(p.y.try_into().unwrap(), p.x.try_into().unwrap(), 'O'); 
        } else {
            matrix.set(p.y.try_into().unwrap(), p.x.try_into().unwrap(), 'o');
        }
    }



    for val in matrix.clone() {
        println!("{} ", val);
    }

    let format = Format::new(1, 1);
    let vec = matrix.to_vec()
    .iter()
    .enumerate()
    .map(|(_, x)| {
        let ansi_fg = 15;
        let ansi_bg = 0;
        cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
        })
    .collect::<Vec<_>>();
    let mut data = matrix::Matrix::new(MAP_WIDTH, vec);
    let display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::Plain);
    
}

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}


fn spawn_stdin_channel() -> crossbeam_channel::Receiver<char> {
    let (tx, rx) = bounded(100);
    thread::spawn(move || loop {
        let stdout = Term::buffered_stdout();
        let ch = stdout.read_char().unwrap();
        let _res = tx.send(ch);
    });
    rx
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("PRESS any key to start!");
    stdin().read(&mut [0]).unwrap();


    reqwest::get("http://127.0.0.1:8080").await?;


    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_millis(500));

    let stdin_event = spawn_stdin_channel();

    loop {
        select! {
            recv(ticks) -> _ => {
                update_map().await;
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Bye!");
                break;
            }
            recv(stdin_event) -> key => {
                match key {
                    Ok(k) => {
                        match k {
                            'w' => {
                                let client = reqwest::Client::new();
                                client.post("http://127.0.0.1:8080/snake/:up")
                                .body("")
                                .send()
                                .await?;
                            },
                            'a' => {
                                let client = reqwest::Client::new();
                                client.post("http://127.0.0.1:8080/snake/:left")
                                .body("")
                                .send()
                                .await?;
                            },
                            's' => {
                                let client = reqwest::Client::new();
                                client.post("http://127.0.0.1:8080/snake/:down")
                                .body("")
                                .send()
                                .await?;
                            },
                            'd' => {
                                let client = reqwest::Client::new();
                                client.post("http://127.0.0.1:8080/snake/:right")
                                .body("")
                                .send()
                                .await?;
                            }
                            _ => {}
                        }
                    },
                    _ => { }
                }
            }
            default => {
                
            }
        }
    }

    Ok(())
}
