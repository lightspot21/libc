// Convention: If bitness is specified in the original C type,
// then the corresponding Rust type is specified directly.
// (eg. if original is uint64_t, the u64 type is used). If the
// original C type does not specify bitness, then the corresponding
// C mask is used.
// (eg. if original is unsigned long, the c_ulong type will be used here).

// C types not defined by unix family and inferred by limits.h
pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;

// start sys/ioctl.h
extern "C" {
    pub fn ioctl(fd: ::c_int, request: ::c_uint, ...) -> ::c_int;
}
// end sys/ioctl.h

// start sys/ioctl_numbers.h
pub const FIONBIO: ::c_int = 37;
pub const FIOCLEX: ::c_int = 0; // This constant does not exist in SerenityOS.
// end sys/ioctl_numbers.h

// start sys/types.h
pub type socklen_t = u32; //Kernel/API/POSIX/sys/types.h:56
pub type clock_t = u32;
pub type dev_t = u32;
pub type mode_t = u16; 
pub type off_t = i64;
pub type suseconds_t = i32;
pub type time_t = i64;
pub type ino_t = u64;

pub type nlink_t = u32;
pub type blksize_t = u32;
pub type blkcnt_t = u32;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;

pub type pthread_attr_t = *mut ::c_void;
pub type pthread_key_t = ::c_int;
pub type pthread_rwlockattr_t = *mut ::c_void;
pub type pthread_rwlock_t = u64;
pub type pthread_t = ::c_int;
s! {
    pub struct pthread_cond_t {
        pub mutex: *mut ::pthread_mutex_t,
        pub value: u32,
        pub clockid: ::clockid_t //originally int in the header, annotated as clockid_t
    }
    pub struct pthread_condattr_t {
        pub clockid: ::clockid_t //originally int in the header, annotated as clockid_t
    }
    pub struct pthread_mutexattr_t {
        pub r#type: ::c_int
    }
    pub struct pthread_mutex_t {
        pub lock: u32,
        pub owner: ::pthread_t,
        pub level: ::c_int,
        pub r#type: ::c_int
    }
}
// end sys/types.h

// start sys/socket.h
pub type sa_family_t = u16; //Kernel/API/POSIX/sys/socket.h:55

pub const AF_INET: ::c_int = 2;
pub const AF_INET6: ::c_int = 3;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_UNIX: ::c_int = AF_LOCAL;

pub const MSG_PEEK: ::c_int = 0x4;

pub const SHUT_RD: ::c_int = 1;
pub const SHUT_WR: ::c_int = 2;
pub const SHUT_RDWR: ::c_int = 3;

pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOL_SOCKET: ::c_int = 1;

pub const SO_ERROR: ::c_int = 3;
pub const SO_RCVTIMEO: ::c_int = 0;
pub const SO_LINGER: ::c_int = 12;
pub const SO_SNDTIMEO: ::c_int = 1;
s! {
    pub struct sockaddr {
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 14] 
    }
}

extern "C" {
    // original bind doesn't have a name for the third parameter
    pub fn bind(sockfd: ::c_int, addr: *const ::sockaddr, length: ::socklen_t) -> ::c_int;
    pub fn recvfrom(sockfd: ::c_int, buffer: *mut ::c_void, buffer_length: ::size_t, flags: ::c_int, addr: *mut ::sockaddr, addr_length: *mut ::socklen_t) -> ::ssize_t;
}
// end sys/socket.h

// start sys/stat.h
pub const S_IFBLK: ::c_int = 0060000;
pub const S_IFCHR: ::c_int = 0020000;
pub const S_IFDIR: ::c_int = 0040000;
pub const S_IFIFO: ::c_int = 0010000;
pub const S_IFLNK: ::c_int = 0120000;
pub const S_IFMT: ::c_int = 0170000;
pub const S_IFREG: ::c_int = 0100000;
pub const S_IFSOCK: ::c_int = 0140000;
s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atim: ::timespec,
        pub st_mtim: ::timespec,
        pub st_ctim: ::timespec
    }
}
// end sys/stat.h

// start sys/statvfs.h
s! {
    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong
    }
}
// end sys/statvfs.h

// start sys/resource.h
pub type rlim_t = ::size_t;
// end sys/resource.h

// start signal.h
pub type sigset_t = u32;

pub const SIG_SETMASK: ::c_int = 2;
s! {
    pub struct sigaction {
       // pub union u {
       //     sa_handler: *mut fn(::c_int) -> ::c_void,
       //     sa_sigaction: *mut fn(::c_int, *mut ::siginfo_t, *mut ::c_void) -> ::c_void
       // }
        pub sa_mask: ::sigset_t,
        pub sa_flags: ::c_int
    }
}

// end signal.h

// start signal_numbers.h
pub const SIGKILL: ::c_int = 9;
pub const SIGPIPE: ::c_int = 13;
// end signal_numbers.h

// start netinet/in.h
s! {
    pub struct in_addr {
        pub s_addr: u32
    }
    pub struct sockaddr_in {
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8]
    }

    pub struct sockaddr_in6 {
        pub sin6_family: ::sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32
    }
    pub struct sockaddr_storage {
        pub ss_family: ::sa_family_t,
        // pub union {
        //     data: [::c_char; mem::size_of<sockaddr_un>()], //char data[sizeof(struct sockaddr_un)];
        //     alignment: *mut ::c_void
        // }
    }
}
// end netinet/in.h

// start netinet/tcp.h
pub const TCP_NODELAY: ::c_int = 10;
// end netinet/tcp.h

// start sys/uio.h
extern "C" {
    pub fn readv(fd: ::c_int, iov: *const ::iovec, iov_count: ::c_int) -> ::ssize_t;
    pub fn writev(fd: ::c_int, iov: *const ::iovec, iov_count: ::c_int) -> ::ssize_t;
}
// end sys/uio.h

// start sys/un.h
pub const UNIX_PATH_MAX: usize = 108;
s! {
    pub struct sockaddr_un {
        pub sun_family: u16,
        pub sun_path: [::c_char; UNIX_PATH_MAX]
    }
}
// end sys/un.h

// start sys/wait.h
pub const WNOHANG: ::c_int = 1;
safe_f! {
    pub {const} fn WEXITSTATUS(status: ::c_int) -> ::c_int {
        (status & 0xff00) >> 8
    }
    pub {const} fn WSTOPSIG(status: ::c_int) -> ::c_int {
        WEXITSTATUS(status)
    }
    pub {const} fn WTERMSIG(status: ::c_int) -> ::c_int {
        status & 0x7f
    }
    pub {const} fn WIFEXITED(status: ::c_int) -> bool {
        WTERMSIG(status) == 0
    }
    pub {const} fn WIFSTOPPED(status: ::c_int) -> bool {
        (status & 0xff) == 0x7f
    }
    pub {const} fn WIFSIGNALED(status: ::c_int) -> bool {
        (((status & 0x7f) + 1) >> 1) > 0
    }
    pub {const} fn WCOREDUMP(_status: ::c_int) -> bool {
        // This function does not exist in SerenityOS.
        // Assumption: no coredump produced.
        false
    }
    pub {const} fn WIFCONTINUED(_status: ::c_int) -> bool {
        // This function does not exist in SerenityOS.
        // Assumption: the child process was not resumed by delivery of SIGCONT.
        false
    }
}
// end sys/wait.h

// start netdb.h
s! {
    pub struct addrinfo { //Userland/Libraries/LibC/netdb.h
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_addr: *mut ::sockaddr,
        pub ai_canonname: *mut ::c_char,
        pub ai_next: *mut ::addrinfo,
    }
}

pub const EAI_SYSTEM: ::c_int = 11;
// end netdb.h

// start dirent.h
s! {
    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256]
    }
}
// end dirent.h

// start errno_numbers.h
pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const ERANGE: ::c_int = 33;
pub const ENAMETOOLONG: ::c_int = 34;
pub const ELOOP: ::c_int = 35;
pub const EOVERFLOW: ::c_int = 36;
pub const EOPNOTSUPP: ::c_int = 37;
pub const ENOSYS: ::c_int = 38;
pub const ENOTIMPL: ::c_int = 39;
pub const EAFNOSUPPORT: ::c_int = 40;
pub const ENOTSOCK: ::c_int = 41;
pub const EADDRINUSE: ::c_int = 42;
pub const EWHYTHO: ::c_int = 43;
pub const ENOTEMPTY: ::c_int = 44;
pub const EDOM: ::c_int = 45;
pub const ECONNREFUSED: ::c_int = 46;
pub const EADDRNOTAVAIL: ::c_int = 47;
pub const EISCONN: ::c_int = 48;
pub const ECONNABORTED: ::c_int = 49;
pub const EALREADY: ::c_int = 50;
pub const ECONNRESET: ::c_int = 51;
pub const EDESTADDRREQ: ::c_int = 52;
pub const EHOSTUNREACH: ::c_int = 53;
pub const EILSEQ: ::c_int = 54;
pub const EMSGSIZE: ::c_int = 55;
pub const ENETDOWN: ::c_int = 56;
pub const ENETUNREACH: ::c_int = 57;
pub const ENETRESET: ::c_int = 58;
pub const ENOBUFS: ::c_int = 59;
pub const ENOLCK: ::c_int = 60;
pub const ENOMSG: ::c_int = 61;
pub const ENOPROTOOPT: ::c_int = 62;
pub const ENOTCONN: ::c_int = 63;
pub const EPROTONOSUPPORT: ::c_int = 64;
pub const EDEADLK: ::c_int = 65;
pub const ETIMEDOUT: ::c_int = 66;
pub const EPROTOTYPE: ::c_int = 67;
pub const EINPROGRESS: ::c_int = 68;
pub const ENOTHREAD: ::c_int = 69;
pub const EPROTO: ::c_int = 70;
pub const ENOTSUP: ::c_int = 71;
pub const EPFNOSUPPORT: ::c_int = 72;
pub const EDIRINTOSELF: ::c_int = 73;
pub const EDQUOT: ::c_int = 74;
pub const EMAXERRNO: ::c_int = 75;

pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const ELAST: ::c_int = EMAXERRNO;
pub const ESTALE: ::c_int = EMAXERRNO; // This constant does not exist in SerenityOS.
// end errno_numbers.h

// start fcntl.h
pub const AT_FDCWD: ::c_int = -100;
pub const F_DUPFD: ::c_int = 0;
pub const F_DUPFD_CLOEXEC: ::c_int = F_DUPFD; // This constant does not exist in SerenityOS.
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;

pub const O_ACCMODE: ::c_int = O_RDONLY | O_WRONLY;
pub const O_APPEND: ::c_int = 1 << 7;
pub const O_CLOEXEC: ::c_int = 1 << 11;
pub const O_CREAT: ::c_int = 1 << 3;
pub const O_EXCL: ::c_int = 1 << 4;
pub const O_NONBLOCK: ::c_int = 1 << 8;
pub const O_RDONLY: ::c_int = 1 << 0;
pub const O_RDWR: ::c_int = O_RDONLY | O_WRONLY;
pub const O_TRUNC: ::c_int = 1 << 6;
pub const O_WRONLY: ::c_int = 1 << 1;

// end fcntl.h

// start fd_set.h
pub const FD_SETSIZE: usize = 1024;
s! {
    pub struct fd_set {
        pub fds_bits: [::c_uchar; FD_SETSIZE / 8]
    }
}
// end fd_set.h

// start limits.h
pub const PTHREAD_STACK_MIN: ::c_int = 65536;
// end limits.h

// start locale.h
s! {
    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char
    }
}
// end locale.h

// start pwd.h
s! {
    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char
    }
}

extern "C" {
    pub fn getpwuid_r(uid: ::uid_t, pwd: *mut passwd, buf: *mut ::c_char, buflen: ::size_t, result: *mut *mut passwd) -> ::c_int;
}
// end pwd.h

// start poll.h
pub type nfds_t = ::c_uint;

pub const POLLHUP: ::c_int = 1 << 4;
pub const POLLIN: ::c_int = 1 << 0;
pub const POLLNVAL: ::c_int = 1 << 5;
pub const POLLOUT: ::c_int = 1 << 2;
// end poll.h

// start pthread_integration.h
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;
pub const PTHREAD_MUTEX_INITIALIZER: ::pthread_mutex_t = pthread_mutex_t {lock: 0 as u32, owner: 0 as ::pthread_t, level: 0 as ::c_int, r#type: PTHREAD_MUTEX_NORMAL};
// end pthread_integration.h

// start stdio.h
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
// end stdio.h

// start stdlib.h
pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
// end stdlib.h

// start termios.h
pub type speed_t = u32;
pub type tcflag_t = u32;
pub const NCCS: usize = 32;

s! {
    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_cc: [::cc_t; NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t
    }
}
// end termios.h

// start time.h
pub type clockid_t = ::c_int;

pub const CLOCK_REALTIME: ::c_int = 0;
pub const CLOCK_MONOTONIC: ::c_int = 1;
pub const CLOCK_MONOTONIC_COARSE: ::c_int = 4;
s! {
    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
    }
}

extern "C" {
    // these arg names do not exist in SerenityOS source
    pub fn clock_gettime(id: ::clockid_t, specptr: *mut ::timespec) -> ::c_int;
}
// end time.h

// start unistd.h
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 7;
pub const _SC_PAGESIZE: ::c_int = 6;
pub const STDERR_FILENO: ::c_int = 2;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDIN_FILENO: ::c_int = 0;

extern "C" {
    pub fn setgroups(size: ::size_t, list: *const ::gid_t) -> ::c_int;
}
// end unistd.h

// start wchar.h
// CAUTION: Defined this based on GCC's (and clang's) default definition on x86_64-unknown-linux-gnu.
// May be wrong assumption!
pub type wchar_t = ::c_int;
// end wchar.h

// start LibDl/dlfcn.h
pub const RTLD_DEFAULT: ::c_int = 0;
s! {
    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void
    }
}
// end LibDl/dlfcn.h

// start LibPthread/pthread.h
pub const PTHREAD_COND_INITIALIZER: ::pthread_cond_t = pthread_cond_t {mutex: 0 as *mut ::pthread_mutex_t, value: 0 as u32, clockid: CLOCK_MONOTONIC_COARSE};
pub const PTHREAD_RWLOCK_INITIALIZER: ::c_int = 0; // This constant does not exist in SerenityOS.

extern "C" {
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock: ::clockid_t) -> ::c_int;
    pub fn pthread_create(tid: *mut ::pthread_t, attr: *mut ::pthread_attr_t, start: extern "C" fn(*mut ::c_void) -> *mut ::c_void, arg: *mut ::c_void) -> ::c_int;
    pub fn pthread_sigmask(how: ::c_int, set: *const ::sigset_t, old_set: *mut ::sigset_t) -> ::c_int;
}

// end LibPthread/pthread.h

// start LibPthread/semaphore.h
s! {
    pub struct sem_t {
        pub value: u32
    }
}
// end LibPthread/semaphore.h
