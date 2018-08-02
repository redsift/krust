use libc;
use std::fmt;
use std::mem::zeroed;

pub type S = *const libc::c_char;
pub type C = libc::c_char;
pub type G = libc::c_uchar;
pub type H = libc::c_short;
pub type I = libc::c_int;
pub type J = libc::c_longlong;
pub type E = libc::c_float;
pub type F = libc::c_double;
pub type V = libc::c_void;


#[repr(C)]
pub struct K {
    pub m: libc::c_char,
    pub a: libc::c_char,
    pub t: libc::c_char,
    pub u: C,
    pub r: I,
    pub union: [u8; 16],
}

impl fmt::Debug for K {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vs = Vec::new();
        vs.push(format!("Type:{}, Attr:{}, RefCt:{} Addr:{:p}",
               self.t, self.u, self.r, self));
        let mut s = String::new();
        for v in self.union.iter() {
            s.push_str(&format!("{:02x}", v))
        }
        vs.push(format!("Union: 0x{}", s));
        f.write_str(&vs.join("\n"))
    }
}


#[repr(C)]
pub struct U {
    pub g: [G; 16usize],
}

impl ::std::default::Default for U {
    fn default() -> Self { unsafe { zeroed() } }
}
