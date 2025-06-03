#![allow(unused)]

fn contact(){
    let sean: Contact = Contact::from_info("Sean Rutter".to_string(), 2003);
    Contact::print_member_age(&sean);
    println!("{}", sean.get_info());
}

pub struct Contact {
    full_name: String,
    since: u16
}

impl Contact {
    pub fn from_info(full_name: String, since: u16) -> Contact {
        return Contact {full_name, since};
    }

    pub fn print_member_age(cont: &Contact){
        println!("{}, has been a memeber since {}", cont.full_name, cont.since);
    }

    pub fn get_info(&self) -> String{
        format!("{} since : {}", self.full_name, self.since)
    }
}


fn say_hello(){
    let coding :bool = true;
    let mood: &'static str = if coding {"happy"} else {"sad"};

    println!("hello {}", mood);
}

trait BusinessCard {
    fn card(&self) -> String;
}

impl BusinessCard for Contact {
    fn card(&self) -> String {
        format!("{} - Member since: {}", self.full_name, self.since)
    }
}