extern crate enet_sys as enet;
use std::ffi::CString;
use std::ptr;

fn main() {
    if unsafe { enet::enet_initialize() } != 0 {
        println!("error");
    }

    println!("yop la client");
    unsafe { 
        let c = enet::client_new();
        let address = CString::new("localhost".as_bytes()).unwrap();
        let p = enet::client_connect(c, address.as_ptr(), 1234);
            
        enet::handle_event(c);

        if p != ptr::null() {
            enet::host_send(c,p);
        }

        loop {
            enet::handle_event(c);
        }
    }

    unsafe { enet::enet_deinitialize(); }
}


