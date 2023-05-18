mod rusty_shell; // Import the `rusty_shell` module from the `rusty_shell.rs` file
use rusty_shell::SimpleShell; // Import the `SimpleShell` struct from the `rusty_shell` module
use std::ffi::CStr; // Import the `CStr` type from the `std::ffi` module
use std::io::{self, Write}; // Import types related to input/output from the `std::io` module
use std::process::Command; // Import the `Command` struct from the `std::process` module

fn main() {
    // Clean and build the project when running the program
    Command::new("cargo").arg("clean").output().unwrap();
        
    Command::new("cargo").arg("build").output().unwrap();
    
    let mut cmd = String::new(); // Create a mutable string variable to store user input
    let mut cmd_tokens: [*mut i8; 25] = [std::ptr::null_mut(); 25];// Create an array of 25 mutable raw pointers initialized to null. i8 is commonly the Rust type used to represent C characters

    print!("tsh> ");
    io::stdout().flush().unwrap(); // Flush the output to ensure the prompt is displayed immediately

    while let Ok(_) = io::stdin().read_line(&mut cmd) {
        // Read input from the user and check if it was successful

        // Remove the newline character from the command string otherwise subsequent inputs will read wrong.
        cmd = cmd.trim().to_string();

        if !cmd.is_empty() {
            unsafe {
                let cmd_ptr = cmd.as_mut_ptr() as *mut i8; // Convert the command string to a mutable raw pointer.
                let tokens_ptr = cmd_tokens.as_mut_ptr() as *mut *mut i8; // Get a mutable raw pointer to the array of tokens

                // Parse the command into tokens
                SimpleShell::parse_command(cmd_ptr, tokens_ptr);

                // Convert the first token to a string slice. from_ptr allows to safely interpret the C string data as a UTF-8 encoded string in Rust.
                let cmd_str = CStr::from_ptr(*tokens_ptr).to_str().unwrap();

                if SimpleShell::is_quit(cmd_str) {
                    // Check if the command is "quit" or "exit"
                    println!("entered quit loop");
                    std::process::exit(0); // Exit the program
                } else {
                    // Execute the command
                    SimpleShell::exec_command(tokens_ptr);
                }
            }

            // Reset command and command tokens
            cmd.clear(); // Clear the command string
            cmd_tokens = [std::ptr::null_mut(); 25]; // Reset the array of tokens to contain only null pointers
        }

        print!("tsh> "); // Print the shell prompt
        io::stdout().flush().unwrap(); // Flush the output to ensure the prompt is displayed immediately
    }

    println!(); // Print a new line
    std::process::exit(0); // Exit the program
}
