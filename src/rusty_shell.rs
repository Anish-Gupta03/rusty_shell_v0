use std::ffi::{CStr, CString};  // Import the necessary modules for handling C strings
use std::os::raw::{c_char, c_int};  // Import types for C-compatible function signatures
use std::ptr;  // Import the `ptr` module for manipulating raw pointers

pub struct SimpleShell;  

impl SimpleShell {
    pub fn parse_command(cmd: *mut c_char, cmd_tokens: *mut *mut c_char) {
        let c_str = unsafe { CStr::from_ptr(cmd) };  // Create a CStr from the input command
        let cmd_str = c_str.to_str().unwrap();  // Convert the CStr to a Rust string

        let mut i = 0;  // Initialize a counter for token index
        let mut iter = cmd_str.chars();  // Create an iterator over the characters of the command string
        let mut token = String::new();  // Initialize an empty string for storing each token

        // Reset token array
        unsafe {
            for i in 0..25 {
                *cmd_tokens.offset(i) = std::ptr::null_mut();  // Set each element of the token array to null
            }
        }

        // separate the input command string into individual tokens based on whitespace characters, 
        // store each token as a C-compatible string represented by a raw pointer, 
        // and populate the cmd_tokens array with these pointers.
        while let Some(c) = iter.next() {
            if c.is_whitespace() {
                if !token.is_empty() {
                    let token_c = CString::new(token.clone()).unwrap();  // Create a CString from the token string
                    let token_ptr = token_c.into_raw();  // Convert the CString into a raw pointer
                    unsafe {
                        *cmd_tokens.offset(i) = token_ptr;  // Store the token pointer in the token array
                    }
                    i += 1;  // Increment the token index counter
                    token.clear();  // Clear the token string for the next token
                }
            } else {
                token.push(c);  // Add the character to the current token string
            }
        }

        // handle the final token after the while loop has finished the characters in the cmd_str string.
        if !token.is_empty() {
            let token_c = CString::new(token).unwrap();  // Create a CString from the final token string
            let token_ptr = token_c.into_raw();  // Convert the CString into a raw pointer
            unsafe {
                *cmd_tokens.offset(i) = token_ptr;  // Store the token pointer in the token array
            }
            i += 1;  // Increment the token index counter
        }

        unsafe {
            *cmd_tokens.offset(i) = ptr::null_mut();  // Set the last element of the token array to null
        }
    }

    pub fn exec_command(tokens_ptr: *mut *mut c_char) {
        let command = unsafe { *tokens_ptr.offset(0) };  // Get the command from the token array

        let mut args: Vec<*mut c_char> = Vec::new();  // Create a vector to store arguments as raw pointers
        let mut i = 0;  // Initialize a counter for argument index

        unsafe {
            while !(*tokens_ptr.offset(i)).is_null() {
                args.push(*tokens_ptr.offset(i));  // Push each argument pointer into the args vector
                i += 1;  // Increment the argument index counter
            }

            args.push(ptr::null_mut());  // Push a null so its null terminated
    
            let pid = fork();  // Create a child process using the fork() system call

            if pid == 0 {
                execvp(command, args.as_ptr());  // Execute the command with the provided arguments in the child process

                // If the code reaches this point then something went wrong with execvp and we should try again.
                println!("Failed to execute command due to unknown reason. Try again.");
                
                // Reset token array incase previous command arguments are still stored.
                args.clear();
                for i in 0..25 {
                    ptr::write(tokens_ptr.offset(i), ptr::null_mut());  // Set each element of the token array to null
                }
                std::process::exit(1);  // Exit the child process with an error status
            } else {
                waitpid(pid, ptr::null_mut(), 0);  // Wait for the child process to complete in the parent process
            }

            // Clear the args vector for the next command execution
            args.clear();

            // Reset token array
            for i in 0..25 {
                ptr::write(tokens_ptr.offset(i), ptr::null_mut());  // Set each element of the token array to null
            }
        }
    }

    pub fn is_quit(cmd: &str) -> bool {
        let cmd = cmd.trim();  // Trim any leading or trailing whitespace from the input command
        cmd == "quit" || cmd == "exit"  // Check if the command is "quit" or "exit"
    }
}

#[link(name = "c")]
extern "C" {
    fn fork() -> c_int;  // Declare the fork() function from the C library
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;  // Declare the execvp() function from the C library
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;  // Declare the waitpid() function from the C library
}
