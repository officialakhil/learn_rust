use std::fmt::Display;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person: {} ({})", self.name, self.age)
    }
}

pub fn display_person_name_and_age(person: &Person) {
    println!("{} ({})", person.name, person.age);
}
