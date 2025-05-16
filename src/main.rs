use std::{cell::RefCell, io::{stdin, stdout, Write}, rc::{Rc, Weak}};


/// This will define some functions that any object should have, object being anything that can be a filesystem that is visible to the user and will contain other objects/or directly
/// contained by other Obkects 
/// so files and folders, I will use a trait so I can cut down a bit on boilerplate by not having to match and extract the correct type of obejct from every varient of Object
/// The three functions returns properties as denoted in their name that are consistent for every type of object
trait Object {
    //fn get_children(&self) -> Result<Rc<RefCell<Vec<RefCell<FsType>>>>, String>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn get_size(&self) -> usize;
}

/// I have structs for File and Folder, this is a Struct for File which will be passed as a varient to the Object enum later onwards
/// Clone is because when processing files, particulary in Comparisons, I will not need to maintain access to the original value at all times, 
/// Debug for printing its properties and PartialEq for comparisons with other files, attributes can be infered from its properties (fileSize is unique for File)
/// Also note, a File will always be at the bottom of a tree or branch, as files cannot contain
#[derive(Clone, Debug, PartialEq)]
struct File {
    name: String, 
    fileSize: usize,
}

/// Implimentation of File with a constructer function, all property access can be done 
/// via enum factory matching or getting the property from the File directly
impl File {
    fn new(name: String) -> File {
        File {
            name,
            fileSize: 0
        }
    }
}

/// A folder struct, unlike the file struct, this is more useful in node/tree traversals, so a parent and children is a must
/// parent must be a weak unlike children, to avoid cyclic refrences and hence a memory leak/memory which cant be deallocated
/// A parent can be optional, for the example of Root, and folders being processed, RefCells are needed for interior mutability
/// Both for Vector access, and potential access to the metadata of a object
#[derive(Clone, Debug)]
struct Folder
{ 
    name: String,
    parent: Option<Weak<RefCell<Folder>>>,
    children: Rc<RefCell<Vec<RefCell<FsType>>>>
}

/// Implimentation of Folder with a constructer function, all property access can be done 
/// via enum factory matching or getting the property from the Folder directly
/// children can be a empty vec, not needing to be initilized with objects as in the code I never needed to create 
/// children upon the folders first creation
impl Folder{
    fn new(name: String, parent: Option<Weak<RefCell<Folder>>>) -> Folder{
        Folder { 
            name, 
            children: Rc::new(vec![].into()),
            parent,
        }
    }
}

#[derive(Clone, Debug)]
enum FsType {
    File(Rc<RefCell<File>>),
    Folder(Rc<RefCell<Folder>>)
}

/// Implimenting the Object trait for FsType, I will use enum factory matching, to provide unique behavior or behavior at all
/// for certain varients, mainly because I need to de-structure them before I can access any properties, which helps cut down on alot of boilerplate
/// These functions will not be used unless I need access to a specific property or I need to recreate it
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

    /// For simplicity sake I have it return a string for the type of Object, a slightly more advanced way 
    /// would be to use something like !matches(object, Fstype::Varient(_))
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
}

/// List of operations based on whats requried in the assement, text input is remapped into these operations
enum Operations {
    Cd,
    Rm,
    Rmdir,
    Mkdir,
    Touch,
    Ls
}

/// to cut down on boilerplate, a function to display the current location as well as
/// getting and processing input on stdin was deemed nessesary
fn command_line(location: String) -> String {
    // In this instance, print rather than println is nessesary
    // to ensure that the location and prompt (>) is before the text input like a shell
    print!("{}> ", location);
    // Flush previous output so its not detected
    let _ = stdout().flush();

    let mut input = String::new();

    // You only need to read the full line, a full line will only be counted after whats printed because
    // we flusted stdout
    stdin().read_line(&mut input).unwrap();

    // trimming here as there in always whitespace in input which makes it harder to match
    let command = input.trim();
    command.to_owned()
}

/// A filesystem which will hold the root and current_dir, both wrapped in Rc and RefCell so I can access in ways
/// the rust compiler does not support and retain access to the original data at all times
#[derive(Clone)]
struct Filesystem
{
    root: Rc<RefCell<Folder>>,
    current_dir: Rc<RefCell<Folder>>
}

/// Implimentation of filesystem which will be the primary/only way of modifying it internally
/// as well as retriving information about a object
impl Filesystem {
    fn new(root: Rc<RefCell<Folder>>, current_dir: Rc<RefCell<Folder>>) -> Filesystem {
        Filesystem {
            root,
            current_dir,
        }
    }
 
    /// The get function will return a optional folder, wrapped in a Rc RefCell for compatibility reasons
    /// It only does traversals from the root, which given the scale of the project will be sufficent
    /// and will not interfear with the abbility to check from the current directory or absolute paths
    /// it will not return a file, because the purpose of this function is to return a folder to then mutate or list the contents
    /// There is no added benifit for file support
    fn get(&self, path: &str) -> Option<Rc<RefCell<FsType>>> {
        // Clones the root, and wraps it in FsType for compatilibility purposes
        let mut current= Some(FsType::Folder(self.root.clone()));
        // splits the path by / to get each individual item, starting from the beginning and stopping when the final item is found 
        // (also ignoring the empty beginning which might occur with paths like /<path)
        let components = path.split('/').filter(|s| !s.is_empty());
    
        for part in components {
            // Since currant is a FsType, which regardless of how I modify it in the future, 
            // will allow me to immediately disqualify files
            // I can also preform immediate extraction of the folders children vector to try and get the next item, then upon
            // next loop, it will iterate over that
            let next = match current.unwrap() {
                FsType::Folder(folder_rc) => {
                    let borrowed_folder = folder_rc.borrow();
                    let children = borrowed_folder.children.borrow();

                    // find_map as I need to recreate the properly wrapped type, before setting it to next for consistency
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
    
            // I use this instead of ? so I can set currant to the raw Fstype
            match next {
                Some(child) => current = Some(child.borrow().clone()),
                None => return None,
            }
        }
    
        // If no value has been set, meaning the get failed, then it will return none, otherwise it will return it wrapped in the proper
        // smart pointers
        if current.is_some() {
            Some(Rc::new(RefCell::new(current.unwrap())))
        } else {
            None
        }
    }

    /// The create function will take the path needed to get to the final location (including the final location itself)
    /// and the final location as a object, this is because from the path itself I would not be able to infer the final locations object type
    /// I however can infer that any object leading to that point would be a folder, as a file cant contain another object
    fn create(&mut self, path: String, object_type: FsType) {
            
        let path_components: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
    
        
        let mut buffer_dir = self.current_dir.clone();
    
        for path_part in path_components.iter() {
            if path_components.last().unwrap() == path_part {
                match object_type {
                    FsType::File(ref new_file) => {
                        buffer_dir.borrow().children.borrow_mut().push(
                            RefCell::new(FsType::File(new_file.clone())),
                        );
                    }
                    FsType::Folder(ref new_folder) => {
                        buffer_dir.borrow().children.borrow_mut().push(
                            RefCell::new(FsType::Folder(new_folder.clone())),
                        );
                    }
                }
            } else {
                let mut found_folder: Option<Rc<RefCell<Folder>>> = None;
    
                {
                    let buffer_dir_borrow = buffer_dir.borrow();
                    let children = buffer_dir_borrow.children.borrow();
    
                    for child in children.iter() {
                        if let FsType::Folder(existing_folder) = &*child.borrow() {
                            if existing_folder.borrow().name == *path_part {
                                found_folder = Some(existing_folder.clone());
                                break;
                            }
                        }
                    }
                } 
                if let Some(folder) = found_folder {
                    buffer_dir = folder;
                } else {
                    let weak_parent = Rc::downgrade(&buffer_dir);
                    let new_folder = Rc::new(RefCell::new(Folder {
                        name: path_part.clone(),
                        parent: Some(weak_parent),
                        children: RefCell::new(vec![]).into(),
                    }));
    
                    buffer_dir
                        .borrow()
                        .children
                        .borrow_mut()
                        .push(RefCell::new(FsType::Folder(new_folder.clone())));
    
                    buffer_dir = new_folder;
                }
            }
        }
    }

    fn remove(&mut self, path: String, object_type: FsType) {
            
        let path_components: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
    
        
        let mut buffer_dir = self.current_dir.clone();
    
        for path_part in path_components.iter() {
            if path_components.last().unwrap() == path_part {
                match object_type {
                    FsType::File(ref new_file) => {
                        let borrowed_buffer = buffer_dir.borrow();
                        let borrowed_children =  borrowed_buffer.children.borrow_mut();
                        let final_result: Rc<RefCell<Vec<RefCell<FsType>>>> = Rc::new(
                            RefCell::new(
                                borrowed_children.iter().filter(
                                |object| !(object.borrow().get_name() == FsType::File(new_file.clone()).get_name() && object.borrow().get_type() == "file")
                                ).map(|child| child.clone()).collect()
                            )
                        );
                        let borrowed_children_size = borrowed_children.len().clone();
                        drop(borrowed_children);
                        drop(borrowed_buffer);
                        if final_result.borrow().len() != borrowed_children_size { 
                            buffer_dir.borrow_mut().children = final_result;
                        } else {
                            println!("Nothing was removed");
                        }
                    },
                    FsType::Folder(ref new_folder) => {
                        let borrowed_buffer = buffer_dir.borrow();
                        let borrowed_children =  borrowed_buffer.children.borrow_mut();
                        let final_result: Rc<RefCell<Vec<RefCell<FsType>>>> = Rc::new(
                            RefCell::new(
                                borrowed_children.iter().filter(
                                |object| !(object.borrow().get_name() == FsType::Folder(new_folder.clone()).get_name() && object.borrow().get_type() == "folder")
                                ).map(|child| child.clone()).collect()
                            )
                        );
                        let borrowed_children_size = borrowed_children.len().clone();
                        drop(borrowed_children);
                        drop(borrowed_buffer);
                        if final_result.borrow().len() != borrowed_children_size { 
                            buffer_dir.borrow_mut().children = final_result;
                        } else {
                            println!("Nothing was removed");
                        }
                    },
                }
            } else {
                let mut found_folder: Option<Rc<RefCell<Folder>>> = None;
    
                {
                    let buffer_dir_borrow = buffer_dir.borrow();
                    let children = buffer_dir_borrow.children.borrow();
    
                    for child in children.iter() {
                        if let FsType::Folder(existing_folder) = &*child.borrow() {
                            if existing_folder.borrow().name == *path_part {
                                found_folder = Some(existing_folder.clone());
                                break;
                            }
                        }
                    }
                } 
                if let Some(folder) = found_folder {
                    buffer_dir = folder;
                } else {
                    let weak_parent = Rc::downgrade(&buffer_dir);
                    let new_folder = Rc::new(RefCell::new(Folder {
                        name: path_part.clone(),
                        parent: Some(weak_parent),
                        children: RefCell::new(vec![]).into(),
                    }));
    
                    buffer_dir
                        .borrow()
                        .children
                        .borrow_mut()
                        .push(RefCell::new(FsType::Folder(new_folder.clone())));
    
                    buffer_dir = new_folder;
                }
            }
        }
    } 
}

fn command_line_operation(operation: Operations, filesystem: &mut Filesystem, previous_location: String, location_operation: String) -> String {
    let mut final_location: String = "".to_owned(); 
    match operation {
        Operations::Cd => {
            if filesystem.get(&(previous_location.clone() + "/" + &location_operation)).is_some() {
                if let FsType::Folder(folder) = filesystem.get(&(previous_location.clone() + "/" + &location_operation)).unwrap().borrow().clone(){
                    filesystem.current_dir = folder;
                    final_location = previous_location + "/" + &location_operation
                } else {
                    println!("You tried to cd into a file");
                }
            } else {
                println!("Folder does not exist");
            }
        },
        Operations::Rm => {
            let location = location_operation.clone();
            filesystem.remove(location, FsType::File(Rc::new(File::new(location_operation.clone()).into())));
        },
        Operations::Rmdir => {
            let location = location_operation.clone();
            filesystem.remove(location, FsType::Folder(Rc::new(Folder::new(location_operation.split("/").last().unwrap().to_string(), None).into())));
        },
        Operations::Mkdir => {
            if location_operation.len() > 0 && location_operation.len() < 13 {
                let location = location_operation.clone();
                filesystem.create(location, FsType::Folder(Rc::new(Folder::new(location_operation.split("/").last().unwrap().to_string(), None).into())));
            } else {
                println!("Folder name has to be bigger than one character or lower than 13")
            }                
        },
        Operations::Touch => {
            if location_operation.len() > 0 && location_operation.len() < 13 {
                let location = location_operation.clone();
                filesystem.create(location, FsType::File(Rc::new(File::new(location_operation.clone()).into())));
            } else {
                println!("File name has to be bigger than one character or lower than 13")
            }
        },
        Operations::Ls => {
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
                _ = command_line_operation(Operations::Rm, &mut filesystem, current_location.to_string(), new_location.clone());
            },
            "rmdir" => {
                _ = command_line_operation(Operations::Rmdir, &mut filesystem, current_location.to_string(), new_location.clone());
            },
            "exit" => {
        
            }
             _ => println!("Invalid command")
        }
        args = command_line(current_location.clone()).split(" ").map(|s| s.to_string()).collect();
    }
}
