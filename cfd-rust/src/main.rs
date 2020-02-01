extern crate libc;

use libc::{ c_void, c_int, c_char };
use std::ffi::{ CStr, CString };
use std::ptr;

#[link(name = "cfd")]
extern "C" {
  fn CfdCreateAddress(handle: *const c_void,
                     hash_type: c_int,
                     pubkey: *const i8,
                     redeem_script: *const i8,
                     network_type: c_int,
                     address: *mut *mut c_char,
                     locking_script: *mut *mut c_char,
                     p2sh_segwit_locking_script: *mut *mut c_char) -> c_int;
}

pub fn pubkey_test() -> i32 {
    let pubkey_str: &str = "036b67e1bd3bd3efbc37fdc738ab159a4aa527057eae12a0c4b07d3132580dcdfd";
    let pubkey = CString::new(pubkey_str).expect("CString::new failed");
    unsafe {

        //let pubkey: &[u8] = pubkey_str.as_bytes();
        // let mut pubkey: &[u8] = pubkeyb.clone();
        //pubkey[66] = 0;
        // let p_pubkey : *const c_char = pubkey.as_ptr();
        let hash_type : c_int = 4;  // kCfdP2wpkhAddress
        let network_type : c_int = 2;  // kCfdNetworkRegtest

        let mut address: *mut c_char = ptr::null_mut() as *mut c_char;
        let mut locking_script: *mut c_char = ptr::null_mut() as *mut c_char;
        let mut segwit_locking_script: *mut c_char = ptr::null_mut() as *mut c_char;
        let handle: *const c_void = ptr::null();
        let redeem_script: *const i8 = ptr::null();
        let result = CfdCreateAddress(handle,
                        hash_type,
                        pubkey.as_ptr(),
                        redeem_script,
                        network_type,
                        &mut address,
                        &mut locking_script,
                        &mut segwit_locking_script);
        if result == 0 {
            let check_null: *mut c_char = ptr::null_mut() as *mut c_char;
            if address != check_null {
                let c_address: &CStr = CStr::from_ptr(address);
                let s_address = c_address.to_str().unwrap();
                println!("address: {}", s_address);
                libc::free(address as *mut libc::c_void);
            }
            if locking_script != check_null {
                let c_locking_script: &CStr = CStr::from_ptr(locking_script);
                let s_locking_script = c_locking_script.to_str().unwrap();
                println!("locking_script: {}", s_locking_script);
                libc::free(locking_script as *mut libc::c_void);
            }
            if segwit_locking_script != check_null {
                let c_segwit_locking_script: &CStr = CStr::from_ptr(segwit_locking_script);
                let s_segwit_locking_script = c_segwit_locking_script.to_str().unwrap();
                println!("{}", s_segwit_locking_script);
                libc::free(segwit_locking_script as *mut libc::c_void);
            }
        } else {
            println!("CfdCreateAddress NG:{}", result);
        }
        result as i32
    }
}
fn main() {
    println!("Hello, world!");
    let cfdret = pubkey_test();
    println!("pubkey_test = {}", cfdret);
}
