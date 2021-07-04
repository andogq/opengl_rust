use std::time;
const CSI: &str = "\x1B[";

enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37
}

enum Type {
    Debug,
    Info,
    Warn,
    Error
}

const SET_APPEARANCE: &str = "m";
const CLEAR: u8 = 0;

pub enum Level {
    Debug,
    Normal,
    Quiet,
    Mute
}

impl Level {
    pub fn printable(&self, message_type: Type) -> bool {
        match (self, message_type) {
            (Level::Debug, _) => true,
            (Level::Normal, Type::Info | Type::Warn | Type::Error) => true,
            (Level::Quiet, Type::Error) => true,
            (Level::Mute, _) => false,
            _ => false
        }
    }
}
pub struct Logger {
    start: time::Instant,
    level: Level
}

impl Logger {
    pub fn new(level: Level) -> Logger {
        Logger {
            start: time::Instant::now(),
            level
        }
    }

    pub fn debug(&self, message: &str) {
        if self.level.printable(Type::Debug) {
            self.set_color(Color::Blue);
            self.print_mark(Type::Debug);
    
            print!(" ");
    
            self.print_time();
    
            println!(": {}", message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.level.printable(Type::Info) {
            self.set_color(Color::Green);
            self.print_mark(Type::Info);

            print!(" ");

            self.print_time();

            println!(": {}", message);
        }
    }

    pub fn warn(&self, message: &str) {
        if self.level.printable(Type::Warn) {
            self.set_color(Color::Yellow);
            self.print_mark(Type::Warn);

            print!(" ");

            self.print_time();

            println!(": {}", message);
        }
    }

    pub fn error(&self, message: &str) {
        if self.level.printable(Type::Error) {
            self.set_color(Color::Red);
            self.print_mark(Type::Error);

            print!(" ");

            self.print_time();

            print!(": ");

            self.set_color(Color::Red);
            println!("{}", message);
            self.clear_color();
        }
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn print_time(&self) {
        self.set_color(Color::Cyan);
        print!("{:010}s", self.start.elapsed().as_secs_f32());
        self.clear_color();
    }

    fn print_mark(&self, mark: Type) {
        print!("[{}]", match mark {
            Type::Debug => "-",
            Type::Info => "+",
            Type::Warn => "*",
            Type::Error => "!"
        });
    }

    fn set_color(&self, color: Color) {
        print!("{}{}{}", CSI, color as u8, SET_APPEARANCE);
    }

    fn clear_color(&self) {
        print!("{}{}{}", CSI, CLEAR, SET_APPEARANCE);
    }
}