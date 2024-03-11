use std::process::{self, Command};
use std::io::{self, Write};
use std::env::{self, current_exe};
use std::path::Path;
use dirs;
mod config;


fn init_shell(){
    #[cfg(unix)]
    Command::new("clear").status().unwrap();

       println!("{}", config::ASCII_HOME_SCREEN);
}

fn take_user_input() -> String{
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error getting the current directory: {}", e),
    }
    print!("< ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input.trim().to_string()
}
fn execute_system_command(command: &str, arg: Option<&str>) {
    match command {
        "ls" => {
            #[cfg(unix)]
            {
                Command::new("ls")
                    .status()
                    .expect("Failed to execute 'ls'");
            }
           
        },
        "clear" => {
            #[cfg(unix)]
            {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
            }

        },
        "exit" => {
            #[cfg(unix)]
            {
                process::exit(0);
            }
        },
        "cd" =>{
            #[cfg(unix)]
          {
                let target_dir = if let Some("..") = arg {
                    env::current_dir().ok().and_then(|path| path.parent().map(|p| p.to_path_buf())).map_or_else(|| "".to_string(), |p| p.display().to_string())
                } else {
                    // Default to the home directory if no argument is provided, or use the provided argument
                    arg.map(|a| a.to_string()).unwrap_or_else(|| {
                        dirs::home_dir().map_or_else(|| "".to_string(), |p| p.display().to_string())
                    })
                };

                if let Err(e) = env::set_current_dir(Path::new(&target_dir)) {
                    println!("Failed to change directory: {}", e);
                } else {
                    println!("{}", target_dir);
                }
            }
        },
        "help"=>{
            #[cfg(unix)]
            {
                print!("{}", config::HELP_TEXT);
            }
        },
        "knight"=>{
            #[cfg(unix)]
            {
                Command::new("clear").status().unwrap();
                println!("{}", config::ASCII_SECRET);
            }
        },
        
        "pwd" => {
            #[cfg(unix)]
            {
                match current_exe() {
                    Ok(path) => println!("{}", path.display()), // Use `.display()` to get a printable path
                    Err(e) => println!("Error retrieving the current directory: {}", e),
                }
            }
        },


        _ => println!("Unsupported command: {}", command),
    }
}
fn main(){
    init_shell();
    loop {
        let input = take_user_input();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let arg = parts.next();

        // Execute the command
        execute_system_command(command, arg);
    }
}






