use std::io::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;


pub enum Event<T> {
    Timer,
    Key(T),
}

pub fn event_init(stdin: Stdin) -> mpsc::Receiver<Event<Key>> {
    let (tx, rx) = mpsc::channel();
    let tx_cloned = tx.clone();
    thread::spawn(move || timer_thread(tx_cloned));
    thread::spawn(move || key_thread(tx, stdin));
    rx
}

pub fn event_wait(rx: &mpsc::Receiver<Event<Key>>) -> Option<Key> {
    match rx.recv() {
        Ok(event) => match event {
            Event::Timer => None,
            Event::Key(key) => Some(key),
        },
        Err(_e) => {
            None
        }
    }
}

fn timer_thread(tx: mpsc::Sender<Event<Key>>) {
    loop {
        tx.send(Event::Timer).unwrap();
        thread::sleep(Duration::from_millis(100 * 1));
    }
}

fn key_thread(tx: mpsc::Sender<Event<Key>>, stdin: Stdin) {
    for event in stdin.events() {
        let key = match event.unwrap() {
            termion::event::Event::Key(key) => key,
            _ => continue,
        };
        tx.send(Event::Key(key)).unwrap();
    }
}