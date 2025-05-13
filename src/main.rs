use core::panic;
use std::{io::{stdin, stdout, Write}, cell::RefCell};

// 4gb limit

trait Object {
    fn get_children(&self) -> Result<Vec<RefCell<FsType>>, String>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn get_size(&self) -> usize;
}

#[derive(Clone, Debug)]
struct File {
    name: String, 
    fileSize: usize,
}


#[derive(Clone, Debug)]
struct Folder
{ 
    name: String,
    children: Vec<RefCell<FsType>>
}
impl Folder{
    fn new(name: String) -> Folder{
        Folder { 
            name, 
            children: vec![] 
        }
    }
    fn get_children() -> Vec<FsType>{
        todo!()
    }
    fn create(object: FsType){

    }
}

impl Object for FsType {
    fn get_name(&self) -> String {
        match self {
            FsType::File(file) => {
                file.name.clone()
            },
            FsType::Folder(folder) => {
                folder.name.clone()
            },
        }
    }

    fn get_type(&self) -> String {
        match self {
            FsType::File(_) => {
                "file".to_string()
            },
            FsType::Folder(_) => {
                "folder".to_string()
            },
        }
    }

    fn get_size(&self) -> usize {
        todo!()
    }
    fn get_children(&self) -> Result<Vec<RefCell<FsType>>, String> {
        match self {
            FsType::File(_) => Err("Failed".to_owned()),
            FsType::Folder(folder) => Ok(folder.children.clone()),
        }
    }
}

#[derive(Clone, Debug)]
enum FsType {
    File(File),
    Folder(Folder)
}

enum Operations {
    None,
    Cd,
    Rm,
    Rmdir,
    Mkdir,
    Touch,
    Ls
}

fn command_line(location: String) -> String {
    print!("{}> ", location);
    let _ = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let command = input.trim();
    // println!("{}", command);
    command.to_owned()
}

#[derive(Clone)]
struct Filesystem
{
    root: Folder
}
impl Filesystem {
    fn new() -> Filesystem {
        Filesystem {
            root: Folder {
                name: "/".to_string(),
                children: vec![],
            }
        }
    }
    fn has(&self, path: String) -> bool {
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        self.root.children.iter().any(|ref_object| {
            let object = ref_object.borrow();
            if object.get_type() == "folder" {
                object.get_name() == path_components.last().unwrap().clone()
            } else {
                false
            }
        })
    }
    fn get(&self, path: String) -> Option<&RefCell<FsType>> {
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        let all: Option<&RefCell<FsType>> = self.root.children.iter().find(|ref_object| {
            let object = ref_object.borrow();
            if object.get_type() == "folder" {
                true
            } else {
                false
            }
        });
        all
        //all
    }
    fn create(&mut self, path: String){
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        for (index, item) in path_components.iter().enumerate() {
            let item_data = self.get(item.to_string());
            if let Some(ref item_refcell) = item_data {
                if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                    folder.children.push(
                        RefCell::new(FsType::Folder(Folder {
                            name: item.to_string(),
                            children: vec![],
                        }))
                    );
                }
            } else {
                self.root.children.push(
                    RefCell::new(FsType::Folder(Folder {
                        name: item.to_string(),
                        children: vec![]
                    }))
                );
            }
        }
    }
}

fn command_line_operation(operation: Operations, filesystem: &mut Filesystem, previous_location: String, location_operation: String) -> String {
    let mut final_location: String = "".to_owned(); 
    match operation {
        Operations::None => final_location = "/".to_string(),
        Operations::Cd => {
            if filesystem.has(previous_location.clone() + "/" + &location_operation) {
                final_location = previous_location + "/" + &location_operation
            } else {
                println!("Folder does not exist");
            }
        },
        Operations::Rm => {

            },
        Operations::Rmdir => {

            },
        Operations::Mkdir => {
                filesystem.create(location_operation);
            },
        Operations::Touch => {

            },
        Operations::Ls => {
            // println!("{}", previous_location);
            let folder = filesystem.get(previous_location).unwrap();
            // println!("{:#?}", folder);
            let folder_children = folder.borrow().get_children().map_err(|e| panic!("{}", e)).unwrap();
            for item in folder_children {
                println!("Item: {}", item.borrow().get_name());
            }
        },
    }
    final_location
}
//
//
//
//
// Note to self, 12 character long file+folders and 1kib minimum file, 4gb max

fn main() {
    println!("Hello, welcome to the simple file shell! \n Here are the commands: ");
    println!("");
    let mut filesystem: Filesystem = Filesystem::new();
    let first_command = command_line("/".to_owned());
    let mut args: Vec<String> = first_command.split(" ").map(|s| s.to_string()).collect();

    let mut current_location: String = String::new();

    loop {
        let previous_location = current_location.clone();
        current_location = args.get(1).unwrap_or(&current_location.to_string()).to_string();
        match args.get(0).unwrap().as_ref() {
            "cd" => {
                current_location = command_line_operation(Operations::Cd, &mut filesystem, previous_location, current_location.clone());
            },
            "ls" => {
                _ = command_line_operation(Operations::Ls, &mut filesystem, previous_location, current_location.clone());
            },
            "mkdir" => {
                current_location = command_line_operation(Operations::Mkdir, &mut filesystem, previous_location, current_location.clone());

            },
            "touch" => {

            },
            "rm" => {

            },
            "rmdir" => {

            },
            "exit" => {
                
            }
             _ => println!("Invalid command")
        }
        args = command_line(current_location.clone()).split(" ").map(|s| s.to_string()).collect();
    }
}
