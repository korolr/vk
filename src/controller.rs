//! controller.rs
//!
//! Server listening on a port and executing commands for the vk music player

use musicplayer::MusicPlayer;

use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

use slog::Logger;
use utils::*;

pub struct ControllerServer {
  mp: Arc<Mutex<MusicPlayer>>,
  log: Logger
}

// todo check replace logs to prints

trait CommandHandler {
  fn handle(&self, cmd: String);
}

impl CommandHandler for MusicPlayer {
  fn handle(&self, cmd: String) {
    info!(self.log, "Controller got a command: {}", cmd);
    match &*cmd {
      "start" => self.start(),
      "stop" => self.stop(),
      "pause" => self.pause(),
      "resume" => self.resume(),
      "next" => self.next(),
      "prev" => self.prev(),
      "shuffle" => self.shuffle(),
      "repeat" => self.repeat(),
      _ => error!(self.log, "Unknown command: {}", cmd)
    }
  }
}

impl ControllerServer {
  pub fn new(mp: Arc<Mutex<MusicPlayer>>) -> ControllerServer {
    ControllerServer {
      mp: mp,
      log: get_logger().new(o!(WHERE => "controller"))
    }
  }

  fn _get_client_cmd(mut stream: TcpStream) -> String {
    let mut buf = "".to_string();
    let _ = stream.read_to_string(&mut buf);
    buf
  }

  pub fn start(&self, port: u32) {
    let host = "127.0.0.1:".to_string() + &port.to_string();
    let listener = match TcpListener::bind(&*host) {
      Ok(listener) => listener,
      Err(_) => {
        error!(self.log, "Unable to start the music controller server: \
                        127.0.0.1:{}.", port);
        return
      },
    };
    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          let mp = self.mp.clone();
          let log = self.log.clone();
          thread::spawn(move || {
            let cmd = ControllerServer::_get_client_cmd(stream);
            match mp.lock() {
              Ok(mp) => mp.handle(cmd),
              Err(_) => {
                error!(log, "Couldn't lock MusicPlayer while \
                                        handling server message.");
              }
            };
          });
        },
        Err(_) => {
          error!(self.log, "Error occured while handling incomming connection \
                            in the music controller server.");
        }
      }
    }
  }
}

/// Try to connect to local application server and send a message.
pub fn ping_serv(port: u32, cmd: &str) {
  let host = "127.0.0.1:".to_string() + &port.to_string();
  let mut stream = TcpStream::connect(&*host);
  match stream {
    Ok(mut stream) => {
      let msg = cmd.to_string().into_bytes();
      let _ = stream.write(msg.as_slice());
    },
    Err(e) => {
      println!("Unable to connect to the server 127.0.0.1:{}.\n\
                    Make sure the server application is run on this port.",
      port);
    }
  }
}
