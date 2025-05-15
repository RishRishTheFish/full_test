use core::panic;
use std::{cell::{Ref, RefCell, RefMut}, io::{stdin, stdout, Write}, ops::Deref};

// 4gb limit

trait Object {
    fn get_children(&self) -> Result<RefCell<Vec<RefCell<FsType>>>, String>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn get_size(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
struct File {
    name: String, 
    fileSize: usize,
}
impl File {
    fn new(name: String) -> File {
        File {
            name,
            fileSize: 0
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Folder
{ 
    name: String,
    children: RefCell<Vec<RefCell<FsType>>>
}
impl Folder{
    fn new(name: String) -> Folder{
        Folder { 
            name, 
            children: vec![].into() 
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
    fn get_children(&self) -> Result<RefCell<Vec<RefCell<FsType>>>, String> {
        match self {
            FsType::File(_) => Err("Failed".to_owned()),
            FsType::Folder(folder) => Ok(folder.children.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
                children: vec![].into(),
            }
        }
    }
    fn has(&self, path: String) -> bool {
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        self.root.children.borrow().iter().any(|ref_object| {
            let object = ref_object.borrow();
            if object.get_type() == "folder" {
                object.get_name() == path_components.last().unwrap().clone()
            } else {
                false
            }
        })
    }
    fn get(&self, path: String) -> Option<RefCell<FsType>> {
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        let all = self.root.children.borrow().iter().find(|ref_object| {
            let object = ref_object.borrow().clone();
            // println!("{} {}", object.get_name(), *path_components.last().unwrap());
            if *path_components.last().unwrap() == object.get_name() {
                true
            } else {
                false
            }
        }).cloned();
        println!("{:#?}", all);
        all
        //all.as_ref()
    }
    fn remove(&mut self, path: String, object_type: FsType){
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();

        for item in path_components.iter(){
            let item_data = self.get(item.to_string());
            if let Some(ref item_refcell) = item_data {
                match object_type {
                    FsType::File(ref new_file) => {
                        if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            folder.children.borrow_mut().push(
                                RefCell::new(FsType::File(new_file.clone()))
                            );
                        }
                    },
                    FsType::Folder(ref new_folder) => {
                        if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            //let mut folder_children: Vec<RefMut<FsType>> = folder.children.iter().map(|child| child.borrow_mut()).collect();
                            folder.children.borrow_mut().retain(|matching_fstype| {
                                if let FsType::Folder(matching_folder) = matching_fstype.borrow().clone() {
                                    *folder != matching_folder
                                } else {
                                    false
                                }
                            });
                        }
                    },
                }
            } else {
                self.root.children.borrow_mut().push(
                    match object_type {
                        FsType::File(ref new_file) => {
                            RefCell::new(FsType::File(new_file.clone()))
                        },
                        FsType::Folder(ref new_folder) => {
                            RefCell::new(FsType::Folder(new_folder.clone()))
                        },
                    }
                );
            }
        }
    }
    fn create(&mut self, path: String, object_type: FsType){
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();

        for item in path_components.iter(){
            let item_data = self.get(item.to_string());
            if let Some(ref item_refcell) = item_data {
                match object_type {
                    FsType::File(ref new_file) => {
                        if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            folder.children.borrow_mut().push(
                                RefCell::new(FsType::File(new_file.clone()))
                            );
                        }
                    },
                    FsType::Folder(ref new_folder) => {
                        if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            folder.children.borrow_mut().push(
                                RefCell::new(FsType::Folder(new_folder.clone()))
                            );
                        }
                    },
                }
            } else {
                self.root.children.borrow_mut().push(
                    match object_type {
                        FsType::File(ref new_file) => {
                            RefCell::new(FsType::File(new_file.clone()))
                        },
                        FsType::Folder(ref new_folder) => {
                            RefCell::new(FsType::Folder(new_folder.clone()))
                        },
                    }
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
                let location = location_operation.clone();
                filesystem.create(location, FsType::Folder(Folder::new(location_operation.clone())));
        },
        Operations::Touch => {
            let location = location_operation.clone();
            filesystem.create(location, FsType::File(File::new(location_operation.clone())));
        },
        Operations::Ls => {
            let root= &RefCell::new(FsType::Folder((filesystem.root.clone())));
            let mut folder = filesystem.get(previous_location);
            if folder.is_none() {
                folder = Some(root.clone())
            }
            let folder_children = folder.unwrap().borrow_mut().get_children().map_err(|e| panic!("{}", e)).unwrap().borrow().clone();
            for item in folder_children {
                println!("{}", item.borrow().get_name());
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
    let mut new_location: String = String::new();

    loop {
        new_location = args.get(1).map_or("/", |v| v).to_string();
        match args.get(0).unwrap().as_ref() {
            "cd" => {
                current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), new_location.clone());
            },
            "ls" => {
                _ = command_line_operation(Operations::Ls, &mut filesystem, current_location.to_string(), new_location.clone());
            },
            "mkdir" => {
                _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), new_location.clone());
            },
            "touch" => {
                _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), new_location.clone());
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
