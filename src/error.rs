#[cfg(any(target_os = "netbsd", target_os = "openbsd", target_os = "android"))]
use libc::__errno as errno_location;

#[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "redox"))]
use libc::__errno_location as errno_location;

#[cfg(any(target_os = "solaris", target_os = "illumos"))]
use libc::___errno as errno_location;

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
use libc::__error as errno_location;

#[cfg(target_os = "haiku")]
use libc::_errnop as errno_location;

#[cfg(target_os = "vxworks")]
use libc::errnoGet as get_errno;

#[cfg(target_os = "dragonfly")]
unsafe fn get_errno() -> libc::c_int {
    compile_error!(
        "Until rust-lang/rust#29594 is stable, we cannot get the errno value on DragonFlyBSD"
    );
}

#[cfg(not(any(target_os = "vxworks", target_os = "dragonfly")))]
pub unsafe fn get_errno() -> libc::c_int {
    *errno_location()
}
