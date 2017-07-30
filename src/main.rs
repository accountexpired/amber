use std::collections::HashMap;
use std::io;
use std::io::Write;


struct World {
    rooms: Vec<Room>,
}

impl World {
    fn get_room(&self, index: usize) -> &Room {
        match self.rooms.get(index) {
            None => {
                println!("room with id={} does not exit!", index);
                std::process::exit(1);
            },
            Some(room) => room
        }
    }
}

struct Item {
    short_descr: String,
}

impl Item {
    fn new(short_descr: &str) -> Item {
        Item {
            short_descr: short_descr.to_string(),
        }
    }
}

struct Room {
    long_descr: String,
    exits: HashMap<String, usize>,
    items: Vec<Item>,
}

impl Room {
    fn new(long_descr: &str) -> Room {
        Room {
            long_descr: long_descr.to_string(),
            exits: HashMap::new(),
            items: Vec::new(),
        }
    }

    fn display(&self) {
        println!("{}", self.long_descr);

        for item in &self.items {
            println!("{}", item.short_descr);
        }

        print!("Exits: ");
        for (exit, _) in self.exits.iter() {
            print!("{}, ", exit);
        }
        println!();
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn add_exit(&mut self, direction: &str, room_id: usize) {
        self.exits.insert(direction.to_string(), room_id);
    }

    fn has_exit(&self, direction: &str) -> bool {
        self.exits.contains_key(direction)
    }

    fn get_exit(&self, direction: &str) -> usize {
        self.exits.get(direction).map(|&direction| direction).unwrap() // Clone instead of map. Also, don't unwrap.
    }
}

fn create_forest() -> Vec<Room> {
    let mut room1 = Room::new("A green forest.");
    let mut room2 = Room::new("A green forest.");
    let mut room3 = Room::new("A green forest.");

    let iron_ore = Item::new("An iron ore.");
    room2.add_item(iron_ore);

    let copper_ore = Item::new("A copper ore.");
    room3.add_item(copper_ore);

    room1.add_exit("n", 1);
    room2.add_exit("s", 0);
    room2.add_exit("e", 2);
    room3.add_exit("w", 1);

    vec![room1, room2, room3]
}

fn main() {
    let world = World {rooms: create_forest()};

    let mut kbd_input = String::new();

    let mut current_room = world.get_room(0usize);

    current_room.display();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut kbd_input)
            .ok()
            .expect("Failed to read line");

        kbd_input = kbd_input.trim_right().to_string();

        if current_room.has_exit(&kbd_input) {
            let exit = current_room.get_exit(&kbd_input);
            current_room = world.get_room(exit);
            current_room.display();
        } else {
            println!("What?");
        }

        kbd_input.clear();
    }
}
