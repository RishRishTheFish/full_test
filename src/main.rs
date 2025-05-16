use std::{cell::RefCell, io::{stdin, stdout, Write}, rc::{Rc, Weak}};
use std::str::FromStr;

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
    fn new(name: String, fileSize: usize) -> File {
        File {
            name,
            fileSize
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
        match self {
            FsType::File(file) => file.borrow().fileSize,
            FsType::Folder(folder) => 1,
        }
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
            
        // I split path_components so I can process every folder until the final item, unlike get, I do not filter empty strings, as its useful 
        // in setting the current directory to root
        let mut path_components: Vec<String> = path
            .split('/')
            .map(|s| s.to_string())
            .collect();
 
        // Like current for get, I have a buffer dir because you can navigate relitive to current_dir in a variety of ways, and while traversing to make a object
        // you dont want to be moved through multiple directories to the new one, or however far it goes, so this will not reflect on the current directory
        // also, this will support either the current directory or root if the first item is a /, which will make the first empty, and pick based on that
        let mut buffer_dir = if path_components.first().unwrap().is_empty() {
            self.root.clone()
        } else {
            self.current_dir.clone()
        };
        // then remove the first empty string otherwise it will attempt to create a empty folder
        path_components.retain(|s| !s.is_empty());
    
        // Go over the path components but this time I will also count the index as the last path_part wont match the last path_component so I have to do index tracking
        for (index, path_part) in path_components.iter().enumerate() {
            // if I am not on the last path, then I will create a new folder, otherwise it will create the given object type
            if index == path_components.len() - 1 {
                buffer_dir.borrow().children.borrow_mut().push(object_type.clone().into());
            } else {
                // set a found folder to be optional with the standard smart pointers, so if its not found
                // no bad operation will be attemoted on it
                let mut found_folder: Option<Rc<RefCell<Folder>>> = None;
    
                // Put it in a scope so buffer_dir_borrow and children are dropped, as you cant re-assign to refmuts
                // that are currently being borrowed, the same effect can be used with the drop function
                {
                    // let bindings so the borrow lives for the intended duration
                    let buffer_dir_borrow = buffer_dir.borrow();
                    let children = buffer_dir_borrow.children.borrow();
    
                    // Currently we are only looking for folders so it will try to get the type from folders then match the name, makes sure found_folder is set
                    // to the folder that matches a path part, then breaks the loop
                    for child in children.iter() {
                        if let FsType::Folder(existing_folder) = &*child.borrow() {
                            if existing_folder.borrow().name == *path_part {
                                found_folder = Some(existing_folder.clone());
                                break;
                            }
                        }
                    }
                } 
                // if a part was found, it will set the buffer dir
                if let Some(folder) = found_folder {
                    buffer_dir = folder;
                } else {    
                    // otherwise it will make a new folder
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

    /// This function is structured alot like the create function, however due to alot of diffrences, they cannot be compressed into two small functions and one big function
    /// the arguments are the same with the same purpose except to remove, and object type will be the type matched for removal
    /// any comments here will point out diffrences
    fn remove(&mut self, path: String, object_type: FsType) {

        let mut path_components: Vec<String> = path
            .split('/')
            // .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
    
        let mut buffer_dir = if path_components.first().unwrap().is_empty() {
            self.root.clone()
        } else {
            self.current_dir.clone()
        };
        path_components.retain(|s| !s.is_empty());
        
        for (index, path_part) in path_components.iter().enumerate() {
            if index == path_components.len() - 1 {
                let borrowed_buffer = buffer_dir.borrow();
                let borrowed_children =  borrowed_buffer.children.borrow_mut();

                // here for final result, this time around, it exists to provide a vector without the object type, matching it 
                // with both name and type, to ensure the proper object is removed
                let final_result: Rc<RefCell<Vec<RefCell<FsType>>>> = Rc::new(
                    RefCell::new(
                        borrowed_children.iter().filter(
                        |object| !(object.borrow().get_name() == object_type.get_name() && object.borrow().get_type() == object_type.get_type())
                        ).map(|child| child.clone()).collect()
                    )
                );
                // we will clone the original length of the vector representing the buffer dirs contents, for comparison later
                let borrowed_children_size = borrowed_children.len().clone();
                // here, instead of dropping via scope, we drop using the rust functions, as you cant re-assign to refmuts that are currently being borrowed
                // we do this because we will re-assign the children to the buffer dir to be the new one, if something changed
                drop(borrowed_children);
                drop(borrowed_buffer);
                if final_result.borrow().len() != borrowed_children_size { 
                    buffer_dir.borrow_mut().children = final_result;
                } else {
                    // I did not mirror the create portion of this as in rare instances nothing would be removed, this would catch that case
                    println!("Nothing was removed");
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
                    // We should not create paths on the way to remove a object
                    println!("path does not exist for what your trying to remove");
                }
            }
        }
    } 
}

/// command_line_operation will preform the actual operations, its a abstraction layer which will turn the strings into enum varients
/// but the main reason this function is seperate is because of returning the new path, which only one operation does, but would require alot of logic if I were to attempt to
/// do this is main, it will take the operation, whole filesystem, previous_location, which is the current location, like where the user is
/// and the location operation, which is what the user is either cd'ing into or modifying
fn command_line_operation(operation: Operations, filesystem: &mut Filesystem, previous_location: String, location_operation: String, size: usize) -> String {
    // Makes  new vector which will take up the processed path, process as in it will add .., . and other functionality
    let mut parts: Vec<&str> = Vec::new();

    // if it starts with something that signifies a absolute path, it should make a empty vector, otherwise split
    // the current location, aka previous location, into its parts, and filter out the empty ones, including the one at the beginning
    if location_operation.starts_with('/') {
        parts = vec![];
    } else {
        parts = previous_location.split('/').filter(|s| !s.is_empty()).collect();
    }

    // match some of the basic operations, if there is a .., remove the first item from the array, to move back otherwise just add the path
    for part in location_operation.split('/') {
        match part {
            "." => {},
            "" => {},
            ".." => { parts.pop(); },
            path => parts.push(path),
        }
    }

    // join it into a final path
    let resolved_path = format!("{}", parts.join("/"));

    // will be used eventually in the return value
    let mut final_location: String = "".to_owned(); 

    match operation {
        Operations::Cd => {
            // the only operation which truely modifies the current directory
            // if the resolved path is / which is root, clone root to set it as the current dir and set the final location
            // otherwise get the directory to cd into
            if resolved_path == "/" {
                filesystem.current_dir = filesystem.root.clone();
                final_location = "/".to_string();
            } else if let Some(entry) = filesystem.get(&resolved_path) {
                if let FsType::Folder(folder) = entry.borrow().clone() {
                    // found the folder and can now set the current folder to it, and the final path
                    filesystem.current_dir = folder;
                    final_location = resolved_path;
                } else {
                    println!("You tried to cd into a file");
                    final_location = previous_location;
                }
            } else {
                println!("Folder does not exist");
                // for else clasuses like this one, sets the final location to the current location so it isnt reset
                final_location = previous_location;
            }
        },
        Operations::Rm => {
            filesystem.remove(resolved_path.clone(), FsType::File(Rc::new(File::new(resolved_path.clone(), size).into())));
        },
        Operations::Rmdir => {
            filesystem.remove(resolved_path.clone(), FsType::Folder(Rc::new(Folder::new(resolved_path.split("/").last().unwrap().to_string(), None).into())));
        },
        Operations::Mkdir => {
            // boundries so the last part of a path has to be bigger than 0 characters and smaller than 13
            if resolved_path.split("/").last().unwrap().len() > 0 && resolved_path.split("/").last().unwrap().len() < 13 {
                filesystem.create(resolved_path.clone(), FsType::Folder(Rc::new(Folder::new(resolved_path.split("/").last().unwrap().to_string(), None).into())));
            } else {
                println!("Folder name has to be bigger than one character or lower than 13");
            }
        },
        Operations::Touch => {
            if resolved_path.split("/").last().unwrap().len() > 0 && resolved_path.split("/").last().unwrap().len() < 13 {
                filesystem.create(resolved_path.clone(), FsType::File(Rc::new(File::new(resolved_path.clone(), size).into())));
            } else {
                println!("File name has to be bigger than one character or lower than 13");
            }
        },
        Operations::Ls => {
            let mut folder = filesystem.current_dir.clone(); 
            // if the current directory and the one derived from location_operation is the same, no need to look through the filesystem for it
            if resolved_path != previous_location {
                if let Some(fs_obj) = filesystem.get(&resolved_path) {
                    if let FsType::Folder(folder_rc) = fs_obj.borrow().clone() {
                        folder = folder_rc;
                    } else {
                        println!("{} isnt a folder", location_operation);
                        return previous_location;
                    }
                } else {
                    println!("Folder isnt found: {}", location_operation);
                    return previous_location;
                }
            }

            // let bindings for longer lived values (otherwise memory issues)
            let borrowed_folder = folder.borrow();
            let children = borrowed_folder.children.borrow();
            if children.is_empty() {
                println!("(empty)");
            } else {
                for child in children.iter() {
                    let borrowed = child.borrow();
                    println!("{} {}", borrowed.get_name(), borrowed.get_size());
                }
            }
        }
    }

    final_location
}




/// The main function is the entrypoint, it will create the root, set the current dir to the root, make the filesystem, and 
/// have the loop which will catch every operation the user wants to do, as well as re-create the new command line
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

    // its useful to immediately split user input into arguments
    let mut args: Vec<String> = first_command.split(" ").map(|s| s.to_string()).collect();

    let mut current_location: String = String::new();
    let mut new_location: String = String::new();

    let default_size = 1;

    // ALL the required default creations (there is alot)
    _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), "home".to_string(), default_size);
    current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), "home".to_string(), default_size);
    _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), "documents".to_string(), default_size);
    _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), "downloads".to_string(), default_size);
    _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), "photos".to_string(), default_size);
    _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), "music".to_string(), default_size);
    current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), "/home/documents".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "cv.pdf".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "data.dat".to_string(), default_size);
    current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), "/home/music".to_string(), default_size);
    for i in 0..10 {
        _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), format!("{i}.mp3"), default_size);
    }
    current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), "/home/photos".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "passport.jpg".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "photoid.png".to_string(), default_size);
    _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), "japan2026".to_string(), default_size);
    current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), "/home/photos/japan2026".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "tokyo.png".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "kyoto.jpg".to_string(), default_size);
    _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), "miyajima.gif".to_string(), default_size);
    current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), "/".to_string(), default_size);

    loop {
        let new_args: Vec<String> = command_line(current_location.clone()).split(" ").map(|s| s.to_string()).collect();
        let mut final_size = Some(default_size);
        if new_args.get(2).is_some(){
            let size = usize::from_str(new_args.get(2).unwrap());
            if size.is_ok() {
                final_size = Some(size.unwrap());
            } else {
                println!("You either entered a second argument that was not the file size or you had folders with spaces");
            }
        }

        args = new_args;
        new_location = args.get(1).map_or("/", |v| v).to_string();
        match args.get(0).unwrap().as_ref() {
            "cd" => {
                current_location = command_line_operation(Operations::Cd, &mut filesystem, current_location.to_string(), new_location.clone(), default_size);
            },
            "ls" => {
                _ = command_line_operation(Operations::Ls, &mut filesystem, current_location.to_string(), new_location.clone(), default_size);
            },
            "mkdir" => {
                _ = command_line_operation(Operations::Mkdir, &mut filesystem, current_location.to_string(), new_location.clone(), default_size);
            },
            "touch" => {
                _ = command_line_operation(Operations::Touch, &mut filesystem, current_location.to_string(), new_location.clone(), final_size.unwrap());
            },
            "rm" => {
                _ = command_line_operation(Operations::Rm, &mut filesystem, current_location.to_string(), new_location.clone(), default_size);
            },
            "rmdir" => {
                _ = command_line_operation(Operations::Rmdir, &mut filesystem, current_location.to_string(), new_location.clone(), default_size);
            },
            "exit" => {
        
            }
             _ => println!("Invalid command")
        }
    }
}
