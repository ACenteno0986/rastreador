extern crate libc; 
extern crate nix;
use libc::ptrace;
use libc::fork;
use libc::PTRACE_TRACEME;
use libc::PTRACE_SETOPTIONS;
use libc::PTRACE_O_TRACESYSGOOD;
use libc::PTRACE_SYSCALL;
use libc::WIFSTOPPED;
use libc::WSTOPSIG;
use libc::WIFEXITED;
use libc::PTRACE_PEEKUSER;
use libc::ORIG_RAX;
use libc::waitpid;
use nix::sys::signal::*;
use crate::ForkResult::Parent;
use nix::unistd::*;
use nix::sys::signal::kill;
use nix::sys::ptrace::traceme;
use nix::unistd::getpid;
use nix::unistd::getppid;
use nix::sys::wait;
use libc::execvp;
use nix::unistd::execv;
use nix::sys::ptrace::syscall;
use std::ffi::{CString, CStr};
use std::env;
use std::{ptr};
use std::os::raw::{c_char};


fn to_exec_array<S: AsRef<CStr>>(args: &[S]) -> Vec<*const c_char> {
    use std::iter::once;
    args.iter().map(|s| s.as_ref().as_ptr()).chain(once(ptr::null())).collect()
}

fn wait_for_syscall(pid:i32) -> i32{
    
    let status = 0;
    let mut x = 0;

        unsafe{
            ptrace(PTRACE_SYSCALL, pid, 0, 0);
            waitpid(pid, status as *mut i32, 0);
            
            if WIFSTOPPED(status) && (WSTOPSIG(status) == 0x80 ){
                x = 1;
            }
            
            if WIFEXITED(status){
                x = 0;
            }
        }
        return x;
}

fn nuevo_hijo(args:Vec<String>) -> i32{
    
    
    let raw = &args[0].as_bytes();
    let mut vec = Vec::new();
    let arg;
    unsafe{
        arg = CString::from_vec_unchecked(raw.to_vec());
            
    }
        
    for i in 1..args.len(){
            
        unsafe{
            vec.push(CString::from_vec_unchecked(args[i].as_bytes().to_vec()));
            
        }
    }
        
    let args_p = to_exec_array(&vec);
    let pid;
    unsafe {
        ptrace(PTRACE_TRACEME);
        kill(getpid(), SIGSTOP);
        pid = execvp(arg.as_ptr(), args_p.as_ptr());
            
    }
    println!("Test2: {}", pid);
    pid
    
}

fn steped(pid:i32){
    
 
            
        println!("Test2");
            
    
}



fn fluid(pid:i32){
    
    let status = 0;
    let mut syscall = 0;
    unsafe{
        waitpid(pid, status as *mut i32, 0);
        ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACESYSGOOD);
    }
    
    while(true){
        
        if wait_for_syscall(pid) != 0 {
            break;
            
        } 
        
        unsafe{
            syscall = ptrace(PTRACE_PEEKUSER, pid, ORIG_RAX);
            println!("Syscall: {}", syscall);
        }
        
        if wait_for_syscall(pid) != 0 {
            break;
        }
    }
    
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
        
        let hijo;
        unsafe{
          
            hijo = fork();
        }
        
        if hijo == 0 {
            nuevo_hijo(args);
            
        }
        else{
            
            if arg1 == "-v" || arg1 == "-V"{

                fluid(hijo);
        
                println!("{},{}", arg1, arg2);
            }
        }
      
    }
    //println!("{},{}", arg1, arg2);
}