use std::collections::HashMap;
use std::io;
use std::io::Write;

extern crate colored;

use colored::*;

// TODO: Show the room ID for the room you are in.

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

    fn get_room_mut(&mut self, index: usize) -> &mut Room {
        match self.rooms.get_mut(index) {
            None => {
                println!("room with id={} does not exit!", index);
                std::process::exit(1);
            },
            Some(room) => room
        }
    }
}

struct Organism {
    name: String,
    health: i32,
    mental: i32,
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
    organisms: Vec<Organism>,
}

impl Room {
    fn new(long_descr: &str) -> Room {
        Room {
            long_descr: long_descr.to_string(),
            exits: HashMap::new(),
            items: Vec::new(),
            organisms: Vec::new(),
        }
    }

    fn display(&self) {
        println!("{}", self.long_descr);

        for item in &self.items {
            println!("{}.", item.short_descr.green());
        }

        for organism in &self.organisms {
            println!("{}.", organism.name.yellow());
        }

        print!("Exits: ");
        let last_element = self.exits.keys().count();
        for (i, exit) in self.exits.keys().enumerate() {
            if i == (last_element - 1) {
                println!("{}.", exit);
            } else {
                print!("{}, ", exit);
            }
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    fn steal_most_recent_organism(&mut self) -> Organism {
        self.organisms.pop().unwrap()
    }

    fn add_exit(&mut self, direction: &str, room_id: usize) {
        self.exits.insert(direction.to_string(), room_id);
    }

    fn has_exit(&self, direction: &str) -> bool {
        self.exits.contains_key(direction)
    }

    fn get_exit(&self, direction: &str) -> usize {
        self.exits.get(direction).cloned().unwrap()
    }
}

fn create_forest() -> Vec<Room> {
    let mut room1 = Room::new("A green forest. There are trees and bushes all over the place.");

    let mut room2 = Room::new("A green forest. There are few trees here.");

    let mut room3 = Room::new("A green forest. Birds are chirping and the sunlight penetrates\n\
                               the canopy forming a pattern of light on the ground.");

    let iron_ore = Item::new("An iron ore");
    room2.add_item(iron_ore);

    let copper_ore = Item::new("A copper ore");
    let charles = Organism {name: "Charles".to_string(), health: 50, mental: 100};
    room3.add_item(copper_ore);
    room3.add_organism(charles);

    room1.add_exit("n", 1);
    room2.add_exit("s", 0);
    room2.add_exit("e", 2);
    room3.add_exit("w", 1);

    vec![room1, room2, room3]
}

fn main() {
    let mut player = Organism {name: "Yourself".to_string(), health: 100, mental: 100};
    let mut world = World {rooms: create_forest()};

    let mut kbd_input = String::new();

    let mut current_room_id = 0;
    let mut changed_rooms = true;

    loop {
        let mut current_room = world.get_room_mut(current_room_id);
        current_room.add_organism(player);

        if changed_rooms {
            current_room.display();
            changed_rooms = false;
        }

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut kbd_input)
            .expect("Failed to read line");

        kbd_input = kbd_input.trim_right().to_string();

        if current_room.has_exit(&kbd_input) {
            current_room_id = current_room.get_exit(&kbd_input);
            changed_rooms = true;
        } else {
            println!("What?");
        }

        player = current_room.steal_most_recent_organism();

        kbd_input.clear();
    }
}
