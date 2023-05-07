use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
use tokenizers::tokenizer::Tokenizer;

#[no_mangle]
pub extern "C" fn C_tokenizer_from_file(cfile: *const c_char) -> *const c_void {
    let file = unsafe { CStr::from_ptr(cfile).to_str().unwrap() };
    let file = String::from(file);

    let tokenizer = Tokenizer::from_file(file);

    let tokenizer = match tokenizer {
        Ok(tokenizer) => tokenizer,
        Err(e) => {
            println!("Error: {}", e);
            return std::ptr::null_mut();
        }
    };

    return Box::into_raw(Box::new(tokenizer)) as *const c_void;
}

#[no_mangle]
pub extern "C" fn C_tokenizer_from_str(cbytes: *const c_char) -> *const c_void {
    let str = unsafe { CStr::from_ptr(cbytes).to_str().unwrap() };
    let bytes = str.as_bytes();

    let tokenizer = Tokenizer::from_bytes(bytes);

    let tokenizer = match tokenizer {
        Ok(tokenizer) => tokenizer,
        Err(e) => {
            println!("Error: {}", e);
            return std::ptr::null_mut();
        }
    };

    return Box::into_raw(Box::new(tokenizer)) as *const c_void;
}

#[no_mangle]
pub extern "C" fn C_tokenizer_free(tokenizer: *mut c_void) {
    if tokenizer.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(tokenizer as *mut Tokenizer);
    }
}

#[no_mangle]
pub extern "C" fn C_tokenizer_encode_ids(
    tokenizer: *const c_void,
    ctext: *const c_char,
    cadd_special_tokens: bool,
    mut output: *mut u32,
    output_len: i32,
) -> i32 {
    let tokenizer = unsafe { &*(tokenizer as *const Tokenizer) };

    let text = unsafe { CStr::from_ptr(ctext).to_str().unwrap() };
    let text = String::from(text);

    let encoding = tokenizer.encode(text, cadd_special_tokens).unwrap();
    let ids = encoding.get_ids(); 
    
    let mut i: i32 = 0;
    for id in ids {
        if i >= output_len {
            break;
        }
        unsafe {
            *output = *id;
            output = output.offset(1);
        }
        i += 1;
    }

    return i;
}

#[no_mangle]
pub extern "C" fn C_tokenizer_decode(
    tokenizer: *const c_void,
    cids: *const u32,
    cids_len: i32,
    skip_special_tokens: bool,
) -> *const c_char {
    let tokenizer = unsafe { &*(tokenizer as *const Tokenizer) };

    let ids = unsafe { std::slice::from_raw_parts(cids, cids_len as usize) };
    let ids = ids.to_vec();

    let encoding = tokenizer.decode(ids, skip_special_tokens);

    let encoding = match encoding {
        Ok(encoding) => encoding,
        Err(e) => {
            println!("Error: {}", e);
            return std::ptr::null();
        }
    };

    let encoding = encoding.as_str().as_bytes().as_ptr();

    return encoding as *const c_char;
}
