use core::panic;
use std::{cell::{Ref, RefCell, RefMut}, env::current_dir, io::{stdin, stdout, Write}, ops::Deref, rc::Rc};



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

#[derive(Clone, Debug, PartialEq)]
struct Folder
{ 
    name: String,
    children: Rc<RefCell<Vec<RefCell<FsType>>>>
}
impl Folder{
    fn new(name: String) -> Folder{
        Folder { 
            name, 
            children: Rc::new(vec![].into())
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

#[derive(Clone, Debug, PartialEq)]
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
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        self.current_dir.borrow().children.borrow().iter().any(|ref_object| {
            let object = ref_object.borrow();
            if object.get_type() == "folder" {
                object.get_name() == path_components.last().unwrap().clone()
            } else {
                false
            }
        })
    }
    fn get(&self, path: &str) -> Option<Rc<RefCell<FsType>>> {
        // let mut current: Rc<RefCell<FsType>> = self.root;
        let mut current= FsType::Folder(self.root.clone());
        // let FsType::Folder(current) = (*self.root).borrow() {}

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
    
    
    
    // fn remove(&mut self, path: String, object_type: FsType){
    //     let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();

    //     for item in path_components.iter(){
    //         let item_data = self.current_dir;
    //         //self.get(item.to_string());
    //         if let Some(ref item_refcell) = item_data {
    //             match object_type {
    //                 FsType::File(ref new_file) => {
    //                     if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
    //                         folder.borrow().children.borrow_mut().push(
    //                             RefCell::new(FsType::File(new_file.clone()))
    //                         );
    //                     }
    //                 },
    //                 FsType::Folder(ref new_folder) => {
    //                     if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            
    //                         folder.borrow().children.borrow_mut().retain(|matching_fstype| {
    //                             if let FsType::Folder(matching_folder) = matching_fstype.borrow().clone() {
    //                                 *folder != matching_folder
    //                             } else {
    //                                 false
    //                             }
    //                         });
    //                     }
    //                 },
    //             }
    //         } else {
    //             self.root.borrow().children.borrow_mut().push(
    //                 match object_type {
    //                     FsType::File(ref new_file) => {
    //                         RefCell::new(FsType::File(new_file.clone()))
    //                     },
    //                     FsType::Folder(ref new_folder) => {
    //                         RefCell::new(FsType::Folder(new_folder.clone()))
    //                     },
    //                 }
    //             );
    //         }
    //     }
    // }
    fn create(&mut self, path: String, object_type: FsType){
        let path_components: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        //let item = &self.current_dir.borrow().name.clone();
        // for item in path_components.iter(){
            let item_data = Some(self.current_dir.clone());
            if let Some(ref item_refcell) = item_data {
                match object_type {
                    FsType::File(ref new_file) => {
                        //if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            item_refcell.borrow().children.borrow_mut().push(
                                RefCell::new(FsType::File(new_file.clone()))
                            );
                       // }
                    },
                    FsType::Folder(ref new_folder) => {
                        //if let FsType::Folder(ref mut folder) = *item_refcell.borrow_mut() {
                            item_refcell.borrow().children.borrow_mut().push(
                                RefCell::new(FsType::Folder(new_folder.clone()))
                            );
                        //}
                    },
                }
            } else {
                panic!("What happened!");
                // self.root.borrow().children.borrow_mut().push(
                //     match object_type {
                //         FsType::File(ref new_file) => {
                //             RefCell::new(FsType::File(new_file.clone()))
                //         },
                //         FsType::Folder(ref new_folder) => {
                //             RefCell::new(FsType::Folder(new_folder.clone()))
                //         },
                //     }
                // );
            }
        }
    //}
}

fn command_line_operation(operation: Operations, filesystem: &mut Filesystem, previous_location: String, location_operation: String) -> String {
    let mut final_location: String = "".to_owned(); 
    match operation {
        Operations::None => final_location = "/".to_string(),
        Operations::Cd => {
            println!("loc: {}", previous_location.clone() + "/" + &location_operation);
            if filesystem.has(previous_location.clone() + "/" + &location_operation) {
                // if (previous_location.clone() + "/" + &location_operation) == "/" {
                //     println!("root");
                //     filesystem.current_dir = filesystem.root.clone();
                // } else {
                //     println!("non-root");
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
                filesystem.create(location, FsType::Folder(Rc::new(Folder::new(location_operation.clone()).into())));
        },
        Operations::Touch => {
            let location = location_operation.clone();
            filesystem.create(location, FsType::File(Rc::new(File::new(location_operation.clone()).into())));
        },
        Operations::Ls => {
            let mut folder = filesystem.current_dir.clone(); // Default to current dir
        
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
                    println!("{} ({})", borrowed.get_name(), borrowed.get_type());
                }
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
