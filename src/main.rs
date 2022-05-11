use clap::Parser;
use dirs;
use colored::*;
use std::fs::File;
use std::io::prelude::*;
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Action>,
    /// Creates or modifies todo list at specified path
    #[clap(short, long, default_value = "$HOME/.todo")]
    path: String,
}

#[derive(clap::Subcommand)]
enum Action {
    #[clap(about="Adds entry to todo list")]
    Add { entry: Option<String> },
    #[clap(about="Removes entry with id from todo list")]
    Remove { id: Option<u32> },
    #[clap(about="Removes all entries from todo list")]
    Clear,    
    #[clap(about="Marks item with id as done on todo list")]
    Done { id: Option<u32> },
    #[clap(about="Marks item with id as urgent on todo list")]
    Urgent { id: Option<u32> }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut path = args.path;
    if path == "$HOME/.todo" {
        path = dirs::home_dir().unwrap().display().to_string();
        path.push_str("/.todo");
    }
    let mut file: File = match get_file(path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening todo list file: {:?}", e),
    };

    match &args.command {
        Some(Action::Add { entry }) => {
            match entry {
                Some(x) => {
                    match add(file, x.to_string()) {
                        Ok(()) => println!("Successfully added entry."),
                        Err(e) => eprintln!("Failed to add entry: {:?}", e),
                    };
                }
                None => {
                    eprintln!("Error: no todo text to add provided.");
                }
            };
        }
        Some(Action::Remove { id }) => {
            match id {
                Some(x) => {
                    let new_contents = remove(&file, *x)?;
                    file.set_len(0)?;
                    file.write_all(new_contents.as_bytes())?;
                    println!("Successfully removed entry.");
                }
                None => eprintln!("Error: no todo index to remove provided."),
            };
        }
        Some(Action::Clear) => file.set_len(0)?,
        Some(Action::Done { id }) => {
            match id {
                Some(x) => {
                    let new_contents = done(&file, *x)?;
                    file.set_len(0)?;
                    file.write_all(new_contents.as_bytes())?;
                    println!("Successfully updated entry.");
                }
                None => eprintln!("Error: no todo index to update provided."),
            };
        }
        Some(Action::Urgent { id }) => {
            match id {
                Some(x) => {
                    let new_contents = urgent(&file, *x)?;
                    file.set_len(0)?;
                    file.write_all(new_contents.as_bytes())?;
                    println!("Successfully updated entry.");
                }
                None => eprintln!("Error: no todo index to update provided."),
            };
        }
        None => {
            list(&file)?;
        }
    };

    Ok(())
}

fn list(f: &File) -> Result<(), std::io::Error> {
    let mut reader = std::io::BufReader::new(f);
    let mut buf = String::new();
    let mut id = 0;
    while reader.read_line(&mut buf)? > 0 {
        let words = buf.split(" ").collect::<Vec<&str>>();
        let done = words[0] == "DONE";
        let urgent = words[1] == "URGENT";
        if done {
            print!("☑ ");
        } else {
            print!("☐ ");
        }

        let mut text: ColoredString = words[2..].join(" ").red();
        
        if done{
            text = text.green(); 
        }
        else if urgent{
            text = text.bold().italic();
        }
        print!("[{}] {}",id, text);
        buf = String::new();
        id += 1;
    }
    Ok(())
}

fn add(f: File, text: String) -> Result<(), std::io::Error> {
    writeln!(&f, "{} {}", "NOT_DONE NORMAL", text)?;
    Ok(())
}
fn urgent(f: &File, id: u32) -> Result<String, std::io::Error> {
    let mut reader = std::io::BufReader::new(f);
    let mut buf = String::new();
    let mut l = String::new();
    let mut i = 0;
    while reader.read_line(&mut buf)? > 0 {
        if i != id {
            l.push_str(buf.as_str());
        }
        else{
            let split_buf = buf.split(" ").collect::<Vec<&str>>();
                let mut new_contents = String::new();
                new_contents.push_str(split_buf[0]);
            if split_buf[1] == "URGENT" {
               new_contents.push_str(" NORMAL ");
            }
            else{
                new_contents.push_str(" URGENT ");
            }
                new_contents.push_str(&split_buf[2..].join(" "));
                l.push_str(&new_contents);
        }
        i += 1;
        buf.clear();
    }
    if i <= id {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid ID entered",
        ));
    }
    Ok(l)
}
fn done(f: &File, id: u32) -> Result<String, std::io::Error> {
    let mut reader = std::io::BufReader::new(f);
    let mut buf = String::new();
    let mut l = String::new();
    let mut i = 0;
    while reader.read_line(&mut buf)? > 0 {
        if i != id {
            l.push_str(buf.as_str());
        }
        else{
            let split_buf = buf.split(" ").collect::<Vec<&str>>();
                let mut new_contents = String::new();
            if split_buf[0] == "DONE" {
               new_contents.push_str("NOT_DONE ");
            }
            else{
                new_contents.push_str("DONE ");
            }
                new_contents.push_str(&split_buf[1..].join(" "));
                l.push_str(&new_contents);
        }
        i += 1;
        buf.clear();
    }
    if i <= id {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid ID entered",
        ));
    }
    Ok(l)
}

fn remove(f: &File, id: u32) -> Result<String, std::io::Error> {
    let mut reader = std::io::BufReader::new(f);
    let mut buf = String::new();
    let mut l = String::new();
    let mut i = 0;
    while reader.read_line(&mut buf)? > 0 {
        if i != id {
            l.push_str(buf.as_str());
        }
        i += 1;
        buf.clear();
    }
    if i <= id {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid ID entered",
        ));
    }
    Ok(l)
}

fn get_file(fp: String) -> Result<File, std::io::Error> {
    return File::options()
        .read(true)
        .append(true)
        .create(true)
        .open(&fp);
}
