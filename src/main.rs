#![allow(unused)]

use std::collections::HashMap;
use std::io;
use std::ops::Index;

struct Locked;
struct Unlocked;

// Locked door != Unlocked door
#[derive(Debug)]
struct Door<State = Locked> {
    address: String,
    furniture: Vec<Furniture>,
    state: std::marker::PhantomData<State>,
}
#[derive(Debug)]
struct Furniture {
    name: String,
    description: String,
}

impl Furniture {
    pub fn new(name: String, description: String) -> Self {
        println!("We have new furniture!");
        Furniture { name, description }
    }
}

impl Door<Locked> {
    pub fn check(&self, address: String) -> bool {
        if self.address == address {
            println!(" \n \n \n  The correct address!");
            true
        } else {
            println!("Not the correct address!");
            false
        }
    }

    pub fn unlock(self) -> Door<Unlocked> {
        println!("The door has been unlocked! Let's walk inside!");
        Door {
            address: self.address,
            furniture: self.furniture,
            state: std::marker::PhantomData::<Unlocked>,
        }
    }

    pub fn sneak_inside(&self) {
        println!("We've snuck inside...");
        println!("{:?}", self.furniture);
    }
}

impl Door<Unlocked> {
    pub fn lock(self) -> Door<Locked> {
        println!("Walked out and locked the door!");
        Door {
            address: self.address,
            furniture: self.furniture,
            state: std::marker::PhantomData::<Locked>,
        }
    }
    pub fn list_furniture(&self) {
        println!("{:?}", self.furniture);
    }
    pub fn add_furniture(&mut self, name: String, description: String) {
        let furn = Furniture::new(name, description);
        self.furniture.push(furn);
    }
}
// Available for all doors regardless of state
impl<State> Door<State> {
    pub fn safe(&self) -> String {
        //TODO
        let dummy: String = String::from("Dummy");
        dummy
    }
    pub fn insurance(&self) -> String {
        let dummy: String = String::from("Dummy");
        dummy
    }
}

// constructor
impl Door {
    pub fn new(address: String) -> Self {
        Door {
            address,
            furniture: Default::default(),
            state: Default::default(),
        }
    }
}

fn main() {
    let mut clue_count = 0;
    loop {
        let mut tried_address = String::new();
        let mut door = Door::new(String::from("Harringstreet"));

        // Even if you tried, you couldn't door.list_furniture(); at this point....
        // door.list_furniture(); method `list_furniture` not found for this struct
        // YOu need to find the address, and that's when the State of the struct is changed.

        println!("Hello! What address do you want to try?");

        io::stdin()
            .read_line(&mut tried_address)
            .expect("failed to readline");

        let tried_address: String = match tried_address.trim().parse() {
            Ok(string) => string,
            Err(_) => continue,
        };

        if door.check(tried_address) {
            let mut door = door.unlock();
            door.list_furniture();
            door.add_furniture(String::from("shelf"), String::from("brown"));
            door.list_furniture();
            door.add_furniture(String::from("chair"), String::from("yellow"));
            door.list_furniture();

            let mut door = door.lock();
            door.sneak_inside();
            break;
        } else {
            println!("Dmn!");
            if clue_count != door.address.len() {
                clue_count += 1;
                let bytes = door.address.as_bytes();
                println!("{}", &door.address[..clue_count]);
            } else {
                println!("{}", door.address);
            }
        }
    }
}
