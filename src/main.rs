extern crate libc; 
extern crate nix;
use libc::ptrace;
use libc::PTRACE_TRACEME;
use nix::sys::signal::*;
use crate::ForkResult::Parent;
use nix::unistd::{fork, ForkResult};
use nix::unistd::*;
use nix::sys::ptrace;
use nix::sys::signal::kill;
use nix::sys::ptrace::traceme;
use nix::unistd::getpid;
use nix::unistd::getppid;
use nix::sys::wait;
use nix::unistd::execv;
use nix::sys::ptrace::syscall;
use std::ffi::{CString, CStr};
use std::env;
use std::os::raw::{c_char};

fn nuevo_hijo(arg:&CString) -> Pid{
    
    let hijo;
        
    unsafe {
            
        hijo = fork();
           
        println!("Test1: {}", getpid());
        ptrace(PTRACE_TRACEME);
        kill(getpid(), SIGSTOP);
        execvp(&arg, &[&arg]);
            
        println!("Test2: {}", getpid());
            
    }
    
    return getpid();
}

fn steped(arg:&CString){
    
    let hijo;
        
    unsafe {
            
        hijo = fork();
           
        println!("Test1: {}", getpid());
            
        let x = execvp(&arg, &[&arg]);
            
        println!("Test2: {}", getpid());
            
    }
}

fn fluid(syscall:&str){
    //println!("Fluid: {}", syscall);
    
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut args2 : Vec<String> = Vec::new();
    if args.len() > 2{
        let arg1 = &args[1];
        let arg2 = &args[2];
    
            
        if args[1] == "-v" || args[1] == "-V"{
            for x in 1..args.len()-1{
                args2.push(args[x+1].clone());
                
            }
            
        }else{
            for x in 0..args.len()-1{
                args2.push(args[x+1].clone());
                
            }
        }
        
        let raw = &args2[0].as_bytes();
        let argm;
        unsafe{
            argm = CString::from_vec_unchecked(raw.to_vec());
        }
        
        if arg1 == "-v"{
            steped(&argm);
        
        }else if arg1 == "-V"{
            fluid(arg2);
        
        }else{
            
            fluid(arg1);
        }
        
    }else{
        let arg1 = &args[1];
        fluid(arg1);
    }
    //println!("{},{}", arg1, arg2);
}