extern crate loginw;
#[macro_use]
extern crate nix;

use std::{env, str, thread};
use std::ffi::CStr;
use std::io::Write;
use std::time::Duration;
use std::os::unix::io::RawFd;

use loginw::protocol::*;
use loginw::socket::*;

const EVDEV_IOC_MAGIC: char = 'E';
const EVDEV_IOC_GNAME: u8 = 0x06;
ioctl!(read_buf evdev_name with EVDEV_IOC_MAGIC, EVDEV_IOC_GNAME; u8);

fn main() {
    let fd = env::var("LOGINW_FD").expect("No LOGINW_FD, launch under loginw");
    let sock = Socket::new(fd.parse::<RawFd>().expect("parse::<RawFd>()"));
    let mut req = LoginwRequest::new(LoginwRequestType::LoginwOpenInput);
    write!(unsafe { &mut req.dat.bytes[..] }, "/dev/input/event0").expect("write!()");
    sock.sendmsg(&req, None).expect(".sendmsg()");
    let (resp, event0fd) = sock.recvmsg::<LoginwResponse>().expect(".recvmsg()");
    assert!(resp.typ == LoginwResponseType::LoginwPassedFd);
    let mut name_buf = [0u8; 128];
    println!("Read {} bytes from ioctl", unsafe { evdev_name(event0fd.unwrap(), &mut name_buf[..]).unwrap() });
    let name_str = unsafe { CStr::from_ptr(&name_buf[0] as *const u8 as *const _) };
    println!("/dev/input/event0 is a '{}'", str::from_utf8(name_str.to_bytes()).expect("from_utf8()"));
    thread::sleep(Duration::from_secs(2));
}
