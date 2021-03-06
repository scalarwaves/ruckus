/* automatically generated by rust-bindgen */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
    pub struct t_CKCOMPLEX {
        pub re: f64,
        pub im: f64,
    }
    #[test]
    fn bindgen_test_layout_t_CKCOMPLEX() {
        assert_eq!(
            ::std::mem::size_of::<t_CKCOMPLEX>(),
            16usize,
            concat!("Size of: ", stringify!(t_CKCOMPLEX))
        );
        assert_eq!(
            ::std::mem::align_of::<t_CKCOMPLEX>(),
            8usize,
            concat!("Alignment of ", stringify!(t_CKCOMPLEX))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX>())).re as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKCOMPLEX),
                "::",
                stringify!(re)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX>())).im as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKCOMPLEX),
                "::",
                stringify!(im)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
    pub struct t_CKPOLAR {
        pub modulus: f64,
        pub phase: f64,
    }
    #[test]
    fn bindgen_test_layout_t_CKPOLAR() {
        assert_eq!(
            ::std::mem::size_of::<t_CKPOLAR>(),
            16usize,
            concat!("Size of: ", stringify!(t_CKPOLAR))
        );
        assert_eq!(
            ::std::mem::align_of::<t_CKPOLAR>(),
            8usize,
            concat!("Alignment of ", stringify!(t_CKPOLAR))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKPOLAR>())).modulus as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKPOLAR),
                "::",
                stringify!(modulus)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKPOLAR>())).phase as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKPOLAR),
                "::",
                stringify!(phase)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
    pub struct t_CKVEC3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    #[test]
    fn bindgen_test_layout_t_CKVEC3() {
        assert_eq!(
            ::std::mem::size_of::<t_CKVEC3>(),
            24usize,
            concat!("Size of: ", stringify!(t_CKVEC3))
        );
        assert_eq!(
            ::std::mem::align_of::<t_CKVEC3>(),
            8usize,
            concat!("Alignment of ", stringify!(t_CKVEC3))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).x as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC3),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).y as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC3),
                "::",
                stringify!(y)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).z as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC3),
                "::",
                stringify!(z)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
    pub struct t_CKVEC4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
    #[test]
    fn bindgen_test_layout_t_CKVEC4() {
        assert_eq!(
            ::std::mem::size_of::<t_CKVEC4>(),
            32usize,
            concat!("Size of: ", stringify!(t_CKVEC4))
        );
        assert_eq!(
            ::std::mem::align_of::<t_CKVEC4>(),
            8usize,
            concat!("Alignment of ", stringify!(t_CKVEC4))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).x as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC4),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).y as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC4),
                "::",
                stringify!(y)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).z as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC4),
                "::",
                stringify!(z)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).w as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVEC4),
                "::",
                stringify!(w)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct t_CKVECTOR {
        pub N: ::std::os::raw::c_ulong,
        pub values: *mut f64,
    }
    #[test]
    fn bindgen_test_layout_t_CKVECTOR() {
        assert_eq!(
            ::std::mem::size_of::<t_CKVECTOR>(),
            16usize,
            concat!("Size of: ", stringify!(t_CKVECTOR))
        );
        assert_eq!(
            ::std::mem::align_of::<t_CKVECTOR>(),
            8usize,
            concat!("Alignment of ", stringify!(t_CKVECTOR))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVECTOR>())).N as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVECTOR),
                "::",
                stringify!(N)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKVECTOR>())).values as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKVECTOR),
                "::",
                stringify!(values)
            )
        );
    }
    impl Default for t_CKVECTOR {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type c_constr = *const ::std::os::raw::c_char;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
    pub struct t_CKCOMPLEX_SAMPLE {
        pub re: f32,
        pub im: f32,
    }
    #[test]
    fn bindgen_test_layout_t_CKCOMPLEX_SAMPLE() {
        assert_eq!(
            ::std::mem::size_of::<t_CKCOMPLEX_SAMPLE>(),
            8usize,
            concat!("Size of: ", stringify!(t_CKCOMPLEX_SAMPLE))
        );
        assert_eq!(
            ::std::mem::align_of::<t_CKCOMPLEX_SAMPLE>(),
            4usize,
            concat!("Alignment of ", stringify!(t_CKCOMPLEX_SAMPLE))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX_SAMPLE>())).re as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKCOMPLEX_SAMPLE),
                "::",
                stringify!(re)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX_SAMPLE>())).im as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(t_CKCOMPLEX_SAMPLE),
                "::",
                stringify!(im)
            )
        );
    }
    pub type FILE = root::_IO_FILE;
    extern "C" {
        pub fn EM_newline();
    }
    extern "C" {
        pub fn ck_fprintf_stderr(format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn EM_log(arg1: ::std::os::raw::c_long, arg2: root::c_constr, ...);
    }
    extern "C" {
        pub fn EM_setlog(arg1: ::std::os::raw::c_long);
    }
    extern "C" {
        pub fn EM_pushlog();
    }
    extern "C" {
        pub fn EM_poplog();
    }
    extern "C" {
        pub fn EM_error(arg1: ::std::os::raw::c_int, arg2: root::c_constr, ...);
    }
    extern "C" {
        pub fn EM_error2(arg1: ::std::os::raw::c_int, arg2: root::c_constr, ...);
    }
    extern "C" {
        pub fn EM_error2b(arg1: ::std::os::raw::c_int, arg2: root::c_constr, ...);
    }
    extern "C" {
        pub fn EM_error3(arg1: root::c_constr, ...);
    }
    extern "C" {
        pub fn EM_impossible(arg1: root::c_constr, ...);
    }
    extern "C" {
        pub fn EM_reset(filename: root::c_constr, fd: *mut root::FILE) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn EM_change_file(filename: root::c_constr);
    }
    extern "C" {
        pub fn EM_lasterror() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn EM_reset_msg();
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ChucK {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Chuck_Carrier {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Chuck_VM {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Chuck_IO_Serial {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Chuck_Compiler {
        _unused: [u8; 0],
    }
}
