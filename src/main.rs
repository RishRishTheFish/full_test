use std::io::{stdin, stdout, Write};

trait Object {
    fn get_name(&self) -> String;
    fn get_size(&self) -> usize;
}

struct File {
    name: String, 
    fileSize: usize,
}
impl Object for File{
    fn get_name(&self) -> String {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }
}
struct Folder<O> 
where O: Object
{
    name: String,
    children: Vec<O>
}
impl <O: Object>Folder<O>{
    fn get_children() -> Vec<O>{
        todo!()
    }
}
impl <O: Object>Object for Folder<O>{
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> usize {
        self.children.iter().fold(0, |acc, child| {
            acc + child.get_size()
        })
    }
}


fn command_line(current_location: String) -> String {
    print!("{}> ", current_location);
    let _ = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let command = input.trim();
    command.to_owned()
}

// Note to self, 12 character long file+folders and 1kib minimum file, 4gb max

fn main() {
    println!("Hello, welcome to the simple file shell! \n Here are the commands: ");
    println!("");
    let first_command = command_line("\\".to_owned());
    loop {
        match first_command.as_str() {
            "cd" => {

            },
            "ls" => {

            },
            "mkdir" => {

            },
            "touch" => {

            },
            "rm" => {

            },
            "rmdir" => {

            },
             _ => println!("Invalid command")
        }
    }
}
