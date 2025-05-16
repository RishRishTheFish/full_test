use core::panic;
use std::{cell::{Ref, RefCell, RefMut}, env::current_dir, io::{stdin, stdout, Write}, ops::Deref, rc::{Rc, Weak}};



trait Object {
    fn get_children(&self) -> Result<Rc<RefCell<Vec<RefCell<FsType>>>>, String>;
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

#[derive(Clone, Debug)]
struct Folder
{ 
    name: String,
    parent: Option<Weak<RefCell<Folder>>>,
    children: Rc<RefCell<Vec<RefCell<FsType>>>>
}
impl Folder{
    fn new(name: String, parent: Option<Weak<RefCell<Folder>>>) -> Folder{
        Folder { 
            name, 
            children: Rc::new(vec![].into()),
            parent,
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
                file.borrow().name.clone()
            },
            FsType::Folder(folder) => {
                folder.borrow().name.clone()
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
    fn get_children(&self) -> Result<Rc<RefCell<Vec<RefCell<FsType>>>>, String> {
        match self {
            FsType::File(_) => Err("Failed".to_owned()),
            FsType::Folder(folder) => Ok(folder.borrow().children.clone()),
        }
    }
}

#[derive(Clone, Debug)]
enum FsType {
    File(Rc<RefCell<File>>),
    Folder(Rc<RefCell<Folder>>)
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
    root: Rc<RefCell<Folder>>,
    current_dir: Rc<RefCell<Folder>>
}
impl Filesystem {
    fn new(root: Rc<RefCell<Folder>>, current_dir: Rc<RefCell<Folder>>) -> Filesystem {
        Filesystem {
            root,
            current_dir,
        }
    }
    fn has(&self, path: String) -> bool {
        self.get(&path).is_some()
    }
    fn get(&self, path: &str) -> Option<Rc<RefCell<FsType>>> {
        let mut current= FsType::Folder(self.root.clone());
        let components = path.split('/').filter(|s| !s.is_empty());
    
        for part in components {
            let next = match current {
                FsType::Folder(folder_rc) => {
                    let borrowed_folder = folder_rc.borrow();
                    let children = borrowed_folder.children.borrow();
                    children.iter()
                        .find_map(|child_refcell| {
                            let child = child_refcell.borrow();
                            if child.get_name() == part {
                                Some(Rc::new(RefCell::new(child.clone())))
                            } else {
                                None
                            }
                        })
                }
                FsType::File(_) => return None,
            };
    
            match next {
                Some(child) => current = child.borrow().clone(),
                None => return None,
            }
        }
    
        Some(Rc::new(RefCell::new(current)))
    }

    fn create(&mut self, path: String, object_type: FsType) {
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        let mut buffer_dir = self.current_dir.clone();
    
        for path_part in path_components.clone() {
            if path_part == *path_components.last().unwrap() {
                
                match object_type {
                    FsType::File(ref new_file) => {
                        buffer_dir.borrow().children.borrow_mut().push(
                            RefCell::new(FsType::File(new_file.clone()))
                        );
                    },
                    FsType::Folder(ref new_folder) => {
                        buffer_dir.borrow().children.borrow_mut().push(
                            RefCell::new(FsType::Folder(new_folder.clone()))
                        );
                    },
                }
            } else {
                let weak_parent = Rc::downgrade(&buffer_dir);
                let child = Rc::new(RefCell::new(Folder {
                    name: path_part.clone(),
                    parent: Some(weak_parent),
                    children: RefCell::new(vec![]).into(),  
                }));
                buffer_dir.borrow().children.borrow_mut().push(
                    RefCell::new(FsType::Folder(child.clone()))
                );
                buffer_dir = child;
                //println!("{:#?}", buffer_dir);
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
                if let FsType::Folder(folder) = filesystem.get(&(previous_location.clone() + "/" + &location_operation)).unwrap().borrow().clone(){
                    filesystem.current_dir = folder;
                    println!("setting current dir: {:#?}", filesystem.current_dir);
                }
               // }
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
                filesystem.create(location, FsType::Folder(Rc::new(Folder::new(location_operation.split("/").last().unwrap().to_string(), None).into())));
        },
        Operations::Touch => {
            if location_operation.len() < 1 && location_operation.len() < 13 {
                let location = location_operation.clone();
                filesystem.create(location, FsType::File(Rc::new(File::new(location_operation.clone()).into())));
            } else {
                println!("File name has to be bigger than one character or lower than 13")
            }
        },
        Operations::Ls => {
            if location_operation.len() < 1 && location_operation.len() < 13 {
            let mut folder = filesystem.current_dir.clone(); 
                if !location_operation.is_empty() {
                    let target_path = previous_location.clone() + "/" + &location_operation;
                    if let Some(fs_obj) = filesystem.get(&target_path) {
                        if let FsType::Folder(folder_rc) = fs_obj.borrow().clone() {
                            folder = folder_rc;
                        } else {
                            println!("{} is not a folder", location_operation);
                            return previous_location;
                        }
                    } else {
                        println!("Folder not found: {}", location_operation);
                        return previous_location;
                    }
                }
            
                let borrowed_folder = folder.borrow();
                let children = borrowed_folder.children.borrow();
                if children.is_empty() {
                    println!("(empty)");
                } else {
                    for child in children.iter() {
                        let borrowed = child.borrow();
                        println!("{}", borrowed.get_name());
                    }
                }
        } else {
            println!("File name has to be bigger than one character or lower than 13")
        }
        }
        
    }
    final_location
}





fn main() {
    println!("Hello, welcome to the simple file shell! \n Here are the commands: ");
    println!("");
    let root = Rc::new(RefCell::new(Folder {
        name: "/".to_string(),
        children: Rc::new(vec![].into()),
        parent: None,
    }));
    let current_dir = root.clone();
    let mut filesystem: Filesystem = Filesystem::new(root, current_dir);
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
