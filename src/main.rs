use std::env;

#[derive(Debug)]
struct Mailbox {
    storage: Vec<String>,
}

impl Mailbox {
    fn empty() -> Mailbox {
        Mailbox { storage:  vec![] }
    }

    fn get_mail(&mut self) -> Option<String> {
        self.storage.pop()
    }

    fn put_mail(&mut self, message: String) {
        self.storage.push(message);
    }
}

fn main() {
    let maybe_mail = env::args().nth(1);

    let mut mailbox = Mailbox::empty();

    if let Some(mail) = maybe_mail {
        mailbox.put_mail(mail);
    }

    println!("Mailbox: {:?}", mailbox);
}

#[test]
fn test_mailbox() {
    let mut mailbox = Mailbox::empty();
    mailbox.put_mail(String::from("Hello!"));
    let mail = mailbox.get_mail();
    assert_eq!(Some(String::from("Hello!")), mail);
}
