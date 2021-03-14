extern crate libc; 
type pid_t = i32;
use std::os::raw::c_char;
use libc::syscall;
use std::env;
extern "C" {fn getpid() -> pid_t;}


fn  steped(syscall:&str){
    let x;
    unsafe{
        x = getpid();
    }
    
    println!("Steped: {}", x);
}

fn fluid(syscall:&str){
    println!("Fluid: {}", syscall);
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 2{
        let arg1 = &args[1];
        let arg2 = &args[2];
    

        if arg1 == "-v"{
            steped(arg2);
        
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