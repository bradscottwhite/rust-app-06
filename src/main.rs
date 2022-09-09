/**
 * Sixth Rust program
 * Desc: Makes object for a person and people (an array of persons) and prints and modifies them
 * Date: Sep 8, 2022
 */

/*** Person: ***/

pub struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    /*pub fn get_first_name(&self) -> &String {
        &self.first_name
    }*/

    pub fn set_first_name(&mut self) -> &mut String {
        &mut self.first_name
    }
    pub fn set_last_name(&mut self) -> &mut String {
        &mut self.last_name
    }

    pub fn greet(&self) {
        println!("Howdy {} {}!", &self.first_name, &self.last_name);
    }
}

/*** People: ***/

pub struct People {
    people: Vec<Person>
}

impl People {
    pub fn add_person(&mut self, person: Person) {
        self.people.push(person)
    }

    pub fn first_name(&self, index: usize) -> &String {
        &self.people[index].first_name
    }
    pub fn last_name(&self, index: usize) -> &String {
        &self.people[index].last_name
    }

    pub fn set_first_name(&mut self, index: usize, first_name: String) {
        *self.people[index].set_first_name() = first_name
    }
    pub fn set_last_name(&mut self, index: usize, last_name: String) {
        *self.people[index].set_last_name() = last_name
    }

    pub fn greet(&self) {
        for person in &self.people {
            person.greet();
        }
    }
}


fn main() {
    /* Testing Person: */
    let mut me = Person {
        first_name: String::from("Brad"),
        last_name: String::from("White")
    };
    me.greet();

    *me.set_first_name() = String::from("Carter");
    *me.set_last_name() = String::from("Wilkes");
    me.greet();

    /* Testing People: */
    let mut folks = People {
        people: vec![
            Person {
                first_name: String::from("Robert"),
                last_name: String::from("Byrd")
            }, Person {
                first_name: String::from("Charles"),
                last_name: String::from("Carter")
            }
        ]
    };
    folks.greet();

    folks.set_first_name(1, String::from("Charlie"));
    folks.set_last_name(0, String::from("Bird"));
    
    folks.add_person(Person {
        first_name: String::from("Scott"),
        last_name: String::from("White"),
    });

    folks.greet();
    
    println!("{}", folks.first_name(1));
}
