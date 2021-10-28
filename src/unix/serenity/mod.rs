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
        pub clockid: ::c_int //clockid_t
    }
    pub struct pthread_condattr_t {
        pub clockid: ::c_int //clockid_t
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

s! {
    pub struct sockaddr {
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 14] 
    }
}
// end sys/socket.h

// start sys/stat.h
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

// start sys/un.h
pub const UNIX_PATH_MAX: usize = 108;
s! {
    pub struct sockaddr_un {
        pub sun_family: u16,
        pub sun_path: [::c_char; UNIX_PATH_MAX]
    }
}
// end sys/un.h


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

// start fd_set.h
pub const FD_SETSIZE: usize = 1024;
s! {
    pub struct fd_set {
        pub fds_bits: [::c_uchar; FD_SETSIZE / 8]
    }
}
// end fd_set.h

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
// end pwd.h

// start poll.h
pub type nfds_t = ::c_uint;
// end poll.h

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
// end time.h

// start wchar.h
// CAUTION: Defined this based on GCC's default definition on x86_64-unknown-linux-gnu.
// May be wrong assumption!
pub type wchar_t = ::c_int;
// end wchar.h

// start LibDl/dlfcn.h
s! {
    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void
    }
}
// end LibDl/dlfcn.h

// start LibPthread/semaphore.h
s! {
    pub struct sem_t {
        pub value: u32
    }
}
// end LibPthread/semaphore.h
