use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::thread;
use std::sync::{Arc,Mutex};

#[derive(Debug)]
struct Mailbox {
    storage: Mutex<Vec<String>>
}

impl Mailbox {
    fn empty() -> Mailbox {
        Mailbox { storage: Mutex::new(vec![]) }
    }

    fn get_mail(&self) -> Option<String> {
        let mut vector = self.storage.lock().unwrap();
        vector.pop()
    }

    fn put_mail(&self, message: String) {
        let mut vector = self.storage.lock().unwrap();
        vector.push(message);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7200").unwrap();

    let shared_mailbox = Arc::new(Mailbox::empty());

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let mailbox = shared_mailbox.clone();
                thread::spawn(move || {
                    handle(&mailbox, &mut stream)
                });
            }
            Err(e) => {
                println!("Error connecting: {:?}", e);
            }
        }
    }
}

fn handle(mailbox: &Mailbox, stream: &mut TcpStream) {
    let message = read_message(stream);
    match message.trim() {
        "READ" => {
            let maybe_mail = mailbox.get_mail();
            if let Some(mail) = maybe_mail {
                write!(stream, "{}", mail);
            } else {
                write!(stream, "Sorry, no new messages!");
            }
        }
        _ => {
            mailbox.put_mail(message);
            println!("Mailbox contents: {:?}", mailbox);
        }
    }

}

fn read_message(stream: &mut TcpStream) -> String {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    let res = buffered_stream.read_line(&mut read_buffer);
    res.ok().expect("An error occured while reading!");
    read_buffer
}

#[test]
fn test_mailbox() {
    let mut mailbox = Mailbox::empty();
    mailbox.put_mail(String::from("Hello!"));
    let mail = mailbox.get_mail();
    assert_eq!(Some(String::from("Hello!")), mail);
}
