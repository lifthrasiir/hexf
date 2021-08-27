#![no_main]

use std::{ffi::CString, ptr};

use errno::{errno, set_errno, Errno};
use libc::{c_char, strtod, strtof};
use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};

use hexf_parse::{parse_hexf32, parse_hexf64};

#[derive(Debug, Arbitrary)]
enum TargetType {
    F32,
    F64,
}

// IMPORTANT: For the verification with strtof/d to work properly, your locale (LC_ALL) should be set C.UTF-8.
//            Otherwise strtof/d will use your locale's decimal separator.

macro_rules! verify_with_strto_fn {
    ($target_name:ident, $strto_fn:ident, $string_to_parse:ident, $result:ident) => {
        if $string_to_parse.contains("p") || $string_to_parse.contains("P") {
            if let Ok(c_string) = CString::new($string_to_parse.replace('_', "")) {
                unsafe {
                    // strtod will write a *mut c_char to the character past the last interpreted character in here
                    let mut end_ptr = ptr::null_mut();

                    // Reset errno to 0 so we can reliably detect errors later on.
                    set_errno(Errno(0));
                    let strto_fn_result = $strto_fn(c_string.as_ptr(), &mut end_ptr);

                    // See https://en.cppreference.com/w/c/string/byte/strtof
                    // for a description of the behavior of strtof/d functions

                    let strto_fn_errno = errno();
                    if strto_fn_errno != Errno(0) {
                        assert!(
                            $result.is_err(),
                            concat!(stringify!($target_name), " succeeded, but strtod failed with `{}`"),
                            strto_fn_errno
                        );
                        set_errno(Errno(0))
                    } else if *end_ptr != (b'\0' as c_char) {
                        // end_ptr does not point at the null byte terminating `c_string`.
                        // This indicates that strtof/d at most partially parsed the string,
                        // which for us means that it is invalid.
                        assert!(
                            $result.is_err(),
                            concat!(stringify!($target_name), " succeeded, but strtod only parsed up to byte {} of {}"),
                            end_ptr.offset_from(c_string.as_ptr()),
                            c_string.as_bytes().len() - 1
                        );
                    } else if strto_fn_result.is_infinite() {
                        // strtof/d returns -Inf/+Inf on over-/underflows.
                        assert!(
                            $result.is_err(),
                            concat!(stringify!($target_name), " succeeded, but strtod reported an overflow / underflow"),
                        );
                    } else if let Ok(value) = $result {
                        // This will treat -0 the same as 0. This is fine though,
                        // since regular test cases can cover this.
                        assert_eq!(value, strto_fn_result);
                    }
                }
            }
        }
    };
}

fuzz_target!(|data: (TargetType, bool, String)| {
    let (target_type, allow_underscore, string_to_parse) = data;

    match target_type {
        TargetType::F32 => {
            let result = parse_hexf32(&string_to_parse, allow_underscore);

            verify_with_strto_fn!(parse_hexf32, strtof, string_to_parse, result);
        }
        TargetType::F64 => {
            let result = parse_hexf64(&string_to_parse, allow_underscore);

            verify_with_strto_fn!(parse_hexf64, strtod, string_to_parse, result);
        }
    }
});
