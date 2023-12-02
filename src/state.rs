use termion::event::Key;

pub trait State {
    fn update(&mut self, key: Key) -> Option<Box<dyn State>>;
    fn draw(&self) -> String;
}