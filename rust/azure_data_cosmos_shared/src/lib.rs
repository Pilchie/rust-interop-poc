use std::ffi::{c_char, CStr, CString};

#[repr(C)]
pub struct ByteBuffer {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn binary_encode(ptr: *const c_char) -> ByteBuffer {
    let input = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
    let mut result = Vec::new();
    for b in input.as_bytes() {
        result.push(*b);
    }

    let len = result.len();
    let data = result.as_mut_ptr();
    std::mem::forget(result); // so that it is not destructed at the end of the scope
    ByteBuffer { data, len }
}

#[no_mangle]
pub extern "C" fn binary_decode(buffer: &ByteBuffer) -> *const c_char {
    let s = unsafe { std::slice::from_raw_parts(buffer.data, buffer.len) };
    let c_str = CString::new(s).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *const c_char) {
    // Take the ownership back to rust and drop the owner
    let _ = CString::from_raw(ptr as *mut _);
}

#[no_mangle]
pub unsafe extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buffer.data, buffer.len) };
    let s = s.as_mut_ptr();
    unsafe {
        _ = Box::from_raw(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let s = "something\0";
        let result = binary_encode(s.as_ptr());

        // Adding 1 for the null terminator
        assert_eq!(s.len(), result.len + 1);
        assert_eq!('s' as u8, unsafe { *result.data });
        assert_eq!('m' as u8, unsafe { *result.data.offset(2) });
        unsafe { free_byte_buffer(result) };
    }

    #[test]
    fn decode_works() {
        let mut slice = ['s' as u8, 'o' as u8, 'm' as u8];
        let buffer = ByteBuffer {
            data: slice.as_mut_ptr() as *mut u8,
            len: 3,
        };
        let result = binary_decode(&buffer);
        let c_str = unsafe { CStr::from_ptr(result) };
        assert_eq!("som", c_str.to_str().unwrap());
        unsafe { free_string(result) };
    }

    #[test]
    fn round_trip_works() {
        let s = "something\0";
        let result = binary_encode(s.as_ptr());
        let decoded = binary_decode(&result);
        let c_str = unsafe { CStr::from_ptr(decoded) };
        assert_eq!(&s[0..s.len() - 1], c_str.to_str().unwrap());
        unsafe { free_string(decoded) };
        unsafe { free_byte_buffer(result) };
    }
}
