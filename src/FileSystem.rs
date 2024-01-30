
use std::path::{Path, PathBuf};
use std::env;

use std::ffi::OsStr;
use std::fs::{create_dir_all, File};

use crate::Settings::*;

pub struct FileSystem {

}


impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {

        }
    }


    // will check if the files have been created for this world and if not it will create them
    pub fn CheckFileSystem(&mut self) {

        // first check that the data folder exists
        let mut dataDirectory: PathBuf = match self.CheckDataFolder() {
            Ok(path) => { path },
            Err(e) => { panic!("Failed to find data directory: {:?}", e); }
        };

        // now check if this game world has a folder and files, if it doesnt ill make them
        self.CheckGameFiles(dataDirectory);
    }


    pub fn CheckDataFolder(&self) -> Result<PathBuf, PathBuf> {

        let mut dataDirectory: PathBuf = PathBuf::new();

        // Get a path from the executable to the datya folder
        match env::current_exe() {
            Ok(exePath) => {
                // Create a PathBuf from the executable's path
                dataDirectory = PathBuf::from(exePath);

                // Navigate back out until you find the data folder
                dataDirectory.pop(); // Remove the executable name
                dataDirectory.pop(); // Remove release dir
                dataDirectory.pop(); // Remove target dir

                // Append the relative path to the 'data' directory
                dataDirectory.push("data");
            },
            Err(e) => {
                eprintln!("Failed to get current executable path: {}", e);
            }
        }

        if dataDirectory.exists() && dataDirectory.is_dir() {
            Ok(dataDirectory)
        } else {
            Err(dataDirectory)
        }

    }


    pub fn CheckGameFiles(&mut self, mut path: PathBuf) {

        // check if the world folder exists
        path.push("Worlds");
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created Worlds Directory");
                }
                Err(e) => {
                    eprintln!("Failed to create World Directory at path: {:?}", path)
                }
            }
        }

        // now check if there is a folder for this game world
        path.push(worldName);
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created new game world directory: {:?}", worldName);
                }
                Err(e) => {
                    eprintln!("Failed to create new game world directory at path: {:?}", path)
                }
            }
        }


        // TODO: #39 Check for chunk folder 
        path.push("Chunks");
        if !path.exists() {
            // Create the directory if it does not exist
            match create_dir_all(&path) {
                Ok(_) => {
                    println!("Created chunks directory for world: {:?}", worldName);
                }
                Err(e) => {
                    eprintln!("Failed to create chunks directory at path: {:?}", path)
                }
            }
        }

        // chunks created file
        path.pop();
        path.push("ChunksCreated.txt");

        // if the file doesnt exist make it
        if !path.exists() {
            match File::create(&path) {
                Ok(_) => {
                    println!("Created ChunksCreated.txt file for world: {:?}", worldName);
                }
                Err(e) => {
                    eprintln!("Failed to create ChunksCreated.txt at path: {:?}", path)
                }
            }
        }
    }

}