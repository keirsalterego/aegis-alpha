use nix::libc::MS_PRIVATE;
use nix::sched::{clone, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp};
use nix::mount::{mount, MsFlags};
use std::ffi::CString;


const STACK_SIZE: usize = 1024 * 1024; // it will be a 1mb stack for the child process

fn main() {
    println!("[Parent] Aegis Container Runtime is starting ...");

    let mut stack = vec![0; STACK_SIZE];
    let flags = CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS;

    let pid = unsafe {
        clone (
            Box::new(|| child_process()),
            &mut stack,
            flags,
            Some(nix::sys::signal::Signal::SIGCHLD as i32)
        )
    };

    match pid {
        Ok(child_pid) => {
            println!("[Parent] Created container with PID: {}", child_pid);

            let _ = waitpid(child_pid, None);
            println!("[Parent] Container with PID: {} has exited", child_pid);
            
        }
        Err(e) => eprintln!("[Parent] Failed to create container: {}", e),

    }
}

fn child_process() -> isize {
    println!("[Child] I'm alive! preparing for brain transplant");

    let _ = mount (
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        None::<&str>
    );

    let _ = mount (
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>
    );
    let command = CString::new("sh").expect("CString Failed");
    let args = [command.clone()];

    println!("[Child] Executing /bin/sh....");

    match execvp(&command, &args) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("[Child] execvp failed: {}", e);
            -1
        }
    }
}