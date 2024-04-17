// hextolfp - convert an ascii hex string to an l_fp number

use std::{ffi::{c_char, CStr}, str};
use regex::Regex;

#[no_mangle]
pub extern "C" fn chextolfp(c_buf: *const c_char, lfp: &mut u64) -> bool {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let from: &str = c_str.to_str().unwrap();
	return hextolfp(from, lfp);
}

pub fn hextolfp(instr: &str, lfp: &mut u64) -> bool {
    let re = Regex::new(r"(?m)^[ \t\n]*(0[xX]){0,1}([[:xdigit:]]{8})\.?([[:xdigit:]]{8})[ \t\n]*$").unwrap();
    let Some(caps) = re.captures(instr) else { return false; };

    let dec_i = u32::from_str_radix(&caps[2], 16).unwrap();
    let dec_f = u32::from_str_radix(&caps[3], 16).unwrap();

    // *lfp = u64::from(dec_i) << 32 | u64::from(dec_f);
    *lfp = lfpinit_u(dec_i, dec_f);
    return true;
}

pub fn lfpinit_u(sec: u32, frac: u32) -> u64 {
    return u64::from(sec) << 32 | u64::from(frac);
}

#[cfg(test)]
mod tests { // Interesting no tests without the period...
    use super::*;

    // constants should be in test::libntp::
    const HALF: u32 = 2147483648;               // (1 << 31)
    const QUARTER: u32 = 1073741824;            // (1 << 30)
    
    pub fn lfpinit(sec: i32, frac: u32) -> u64 {
        return u64::from(sec as u32) << 32 | u64::from(frac);
    }

    #[test]
    fn illegal_char() {
        let str = "10000000.0000h000"; // Illegal character h.
        let mut actual: u64 = 0;
        assert_eq!(hextolfp(&str, &mut actual), false);
    }

    #[test]
    fn illegal_number_of_integer() {
        let str = "1000000.00000000"; // Missing one digit in integral part.
        let mut actual: u64 = 0;
        assert_eq!(hextolfp(&str, &mut actual), false);
    }

    #[test]
    fn negative_fraction() {
        let str = "ffffffff.40000000"; // -1 + 0.25 decimal
        let mut actual: u64 = 0;
        let expected: u64 = lfpinit(-1, QUARTER);
        assert_eq!(hextolfp(&str, &mut actual), true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn positive_fraction() {
        let str = "00002000.80000000"; // 8192.5 decimal
        let mut actual: u64 = 0;
        let expected: u64 = lfpinit_u(8192, HALF);
        assert_eq!(hextolfp(&str, &mut actual), true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn negative_integer() {
        let str = "ffffffff.00000000"; // -1 decimal
        let mut actual: u64 = 0;
        let expected: u64 = lfpinit(-1, 0);
        assert_eq!(hextolfp(&str, &mut actual), true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn positive_integer() {
        let str = "0000100000000000"; // 16^3
        let mut actual: u64 = 0;
        let expected: u64 = lfpinit_u(4096, 0);
        assert_eq!(hextolfp(&str, &mut actual), true);
        assert_eq!(expected, actual);
    }
}
