#[cfg(feature = "python")]
use pyo3::prelude::*;

use std::ffi::{c_char, CStr, CString};

#[repr(C)]
pub struct ByteBuffer {
    data: *mut u8,
    len: usize,
}

struct BinaryEncoder;

impl BinaryEncoder {
    fn encode(input: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for b in input.as_bytes() {
            result.push(*b);
        }
        result
    }

    fn decode(input: &[u8]) -> String {
        let mut result = String::new();
        for b in input {
            result.push(*b as char);
        }
        result
    }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn python_encode(input: &str) -> Vec<u8> {
    BinaryEncoder::encode(input)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn python_decode(input: Vec<u8>) -> String {
    BinaryEncoder::decode(&input)
}

#[cfg(feature = "python")]
#[pymodule]
fn azure_data_cosmos_shared(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(python_encode, m)?)?;
    m.add_function(wrap_pyfunction!(python_decode, m)?)?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn binary_encode(ptr: *const c_char) -> ByteBuffer {
    let input = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
    let mut result = BinaryEncoder::encode(input);
    let len = result.len();
    let data = result.as_mut_ptr();
    std::mem::forget(result); // so that it is not destructed at the end of the scope
    ByteBuffer { data, len }
}

#[no_mangle]
pub extern "C" fn binary_decode(buffer: &ByteBuffer) -> *const c_char {
    let bytes = unsafe { std::slice::from_raw_parts(buffer.data, buffer.len) };
    let s = BinaryEncoder::decode(bytes);
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
        let input = "something";
        let result = BinaryEncoder::encode(input);
        assert_eq!(input.len(), result.len());
        assert_eq!('s' as u8, result[0]);
        assert_eq!('m' as u8, result[2]);
    }

    #[test]
    fn decode_works() {
        let slice = ['s' as u8, 'o' as u8, 'm' as u8];
        let result = BinaryEncoder::decode(&slice);
        assert_eq!("som", result);
    }

    #[test]
    fn round_trip_works() {
        let orig = "something";
        let result = BinaryEncoder::encode(orig);
        let decoded = BinaryEncoder::decode(&result);
        assert_eq!(orig, decoded);
    }
}
