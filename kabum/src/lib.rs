extern crate libc;

use libc::c_char;
use std::ffi::CString;
use std::ffi::CStr;

#[no_mangle]
pub extern fn soma_marota(x: i32) -> i32 {
    x + 10
}

#[no_mangle]
pub extern fn string_marota() -> *const c_char {
    println!("Produzindo stringue...");
    let x = CString::new("Esta estringue é marota").unwrap();
    let p = x.as_ptr();
    // let x = "Esta estringue é marota\0".as_ptr();
    // println!("{:?}", x);
    std::mem::forget(x);
    p
}

#[no_mangle]
pub unsafe extern fn imprimir_maroto(text: *const c_char) {
    println!("Cheguei aqui para imprimir!!!!!!!");
    let c_str: &CStr = CStr::from_ptr(text);
    let str_slice: &str = c_str.to_str().unwrap();
    let stringue: String = str_slice.to_owned();
    println!("{:?}", c_str);
    println!("{:?}", str_slice);
    println!("{:?}", stringue);
    // let text_: &mut c_char = &mut text;
    // let s = CString::from_raw(text);
    // println!("{}", s);
    println!("eita");
}

#[no_mangle]
pub unsafe extern fn sanity_check() {
    let s = string_marota();
    imprimir_maroto(s);
}
