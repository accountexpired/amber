use std::collections::HashMap;
use std::io;
use std::io::Write;


struct World {
    rooms: Vec<Room>,
}

impl World {
//    fn add_area(&mut self, area: &mut Vec<Room>) {
//        self.rooms.append(area) // Handle potential panic?
//    }

    fn get_room(&self, index: usize) -> &Room {
        &self.rooms[index] // Figure out how to check if this element really exists...
    }
}

struct Room {
    long_descr: String,
    exits: HashMap<String, usize>,
}

impl Room {
    fn new(long_descr: &str) -> Room {
        Room {
            long_descr: long_descr.to_string(),
            exits: HashMap::new(),
        }
    }

    fn display(&self) {
        println!("{}", self.long_descr);

        print!("Exits: ");
        for (exit, _) in self.exits.iter() {
            print!("{}, ", exit);
        }
        println!();
    }

    fn add_exit(&mut self, direction: &str, room_id: usize) {
        self.exits.insert(direction.to_string(), room_id);
    }

    fn has_exit(&self, direction: &str) -> bool {
        self.exits.contains_key(direction)
    }

    fn get_exit(&self, direction: &str) -> usize {
        self.exits.get(direction).map(|&direction| direction).unwrap() // Figure out what map does...
    }
}

fn main() {
    // Move all the room generation stuff into a function of its own.
    let mut room1 = Room::new("A dark room.");
    let mut room2 = Room::new("A light room.");
    let mut room3 = Room::new("A scary room with two white pillars keeping the ceiling away.");

    room1.add_exit("n", 1);
    room2.add_exit("s", 0);
    room2.add_exit("e", 2);
    room3.add_exit("w", 1);

    let world = World {rooms: vec![room1, room2, room3]};

    let mut kbd_input = String::new();

    let mut current_room = world.get_room(0usize);

    loop {
        current_room.display();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut kbd_input)
            .ok()
            .expect("Failed to read line");

        kbd_input = kbd_input.trim_right().to_string();

        if current_room.has_exit(&kbd_input) {
            let exit = current_room.get_exit(&kbd_input);
            current_room = world.get_room(exit);
        } else {
            println!("What?");
        }

        kbd_input.clear();
    }
}
