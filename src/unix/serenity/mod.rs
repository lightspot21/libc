// C types not defined by unix family and inferred by limits.h
pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;


// start sys/types.h
pub type socklen_t = u32; //Kernel/API/POSIX/sys/types.h:56
// end sys/types.h

// start sys/socket.h
pub type sa_family_t = u16; //Kernel/API/POSIX/sys/socket.h:55

s! {
    pub struct sockaddr {
        sa_family: ::sa_family_t,
        sa_data: [::c_char, 14] 
    }
}
// end sys/socket.h

// start netdb.h
s! {
    pub struct addrinfo { //Userland/Libraries/LibC/netdb.h
        ai_flags: ::c_int,
        ai_family: ::c_int,
        ai_socktype: ::c_int,
        ai_protocol: ::c_int,
        ai_addrlen: ::socklen_t,
        ai_addr: *mut ::sockaddr,
        ai_canonname: *mut ::c_char,
        ai_next: *mut ::addrinfo,
    }
}
// end netdb.h
