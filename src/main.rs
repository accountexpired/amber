use std::collections::HashMap;
use std::io;
use std::io::Write;

extern crate uuid;

use uuid::Uuid;

struct Room
{
    id: Uuid,
    long_descr: String,
    exits: HashMap<String, Uuid>,
}

impl Room
{
    fn new(long_descr: &str) -> Room {
        Room {
            id: Uuid::new_v4(),
            long_descr: long_descr.to_string(),
            exits: HashMap::new(),
        }
    }

    fn display(&self) {
        println!("{}", self.long_descr);
    }

    fn set_exit(&mut self, direction: &str, room_id: Uuid) {
        self.exits.insert(direction.to_string(), room_id);
    }
}

fn generate_forest() -> Vec<Room> {
    let mut area: Vec<Room> = Vec::new();

    let mut room1 = Room::new("A dark room.");
    let mut room2 = Room::new("A light room.");

    room1.set_exit("n", room2.id);
    room2.set_exit("s", room1.id);

    area.push(room1);
    area.push(room2);

    area
}

fn main() {
    let mut kbd_input = String::new();

    let forest = generate_forest();
    let current_room = &forest[0];

    current_room.display();

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut kbd_input)
        .ok()
        .expect("Failed to read line");
}
