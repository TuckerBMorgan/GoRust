use std::net::{TcpListener, TcpStream};

const start_server_value: usize = 100;
const start_client_value: usize = 101;

use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use crate::pong::{Message};


#[derive(SystemDesc, Default)]
pub struct MessageSystem {
    client_or_server: usize,
    to_networking_thread: Option<Sender<usize>>,
    from_networking_thread: Option<Receiver<usize>>,
    thread_handle: Option<thread::JoinHandle<()>>
}

impl <'s> System<'s> for MessageSystem {
    type SystemData = (
        WriteStorage<'s, Message>
    );

    fn run(&mut self, (mut messages): Self::SystemData) {
        for message in (&mut messages).join() {
            if message.value == start_server_value {
                self.client_or_server = 1;
                let (to_server, server_reciver) = channel(); 
                let (to_game, game_reciver) = channel();    
                let handle = thread::spawn(move || server_thread(server_reciver, to_game));
                self.to_networking_thread.replace(to_server);
                self.from_networking_thread.replace(game_reciver);
                self.thread_handle.replace(handle);
            }
            else if message.value == start_client_value {
                self.client_or_server = 2;
            }
        }
    }
}


//The server thread will 
//A. Spawn up a server and listen for connections
//B. 
fn server_thread(from_game: Receiver<usize>, to_game: Sender<usize>) {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
}