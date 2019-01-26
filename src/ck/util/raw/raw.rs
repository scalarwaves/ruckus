/* automatically generated by rust-bindgen */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub const _FEATURES_H: u32 = 1;
    pub const __USE_ISOC11: u32 = 1;
    pub const __USE_ISOC99: u32 = 1;
    pub const __USE_ISOC95: u32 = 1;
    pub const __USE_FORTIFY_LEVEL: u32 = 0;
    pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
    pub const _STDC_PREDEF_H: u32 = 1;
    pub const __STDC_IEC_559__: u32 = 1;
    pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
    pub const __STDC_ISO_10646__: u32 = 201706;
    pub const __GNU_LIBRARY__: u32 = 6;
    pub const __GLIBC__: u32 = 2;
    pub const __GLIBC_MINOR__: u32 = 28;
    pub const _SYS_CDEFS_H: u32 = 1;
    pub const __glibc_c99_flexarr_available: u32 = 1;
    pub const __WORDSIZE: u32 = 64;
    pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
    pub const __SYSCALL_WORDSIZE: u32 = 64;
    pub const __HAVE_GENERIC_SELECTION: u32 = 1;
    pub const __USE_EXTERN_INLINES: u32 = 1;
    pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
    pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
    pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
    pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
    pub const _STDLIB_H: u32 = 1;
    pub const __HAVE_FLOAT128: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
    pub const __HAVE_FLOAT64X: u32 = 1;
    pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
    pub const __HAVE_FLOAT16: u32 = 0;
    pub const __HAVE_FLOAT32: u32 = 1;
    pub const __HAVE_FLOAT64: u32 = 1;
    pub const __HAVE_FLOAT32X: u32 = 1;
    pub const __HAVE_FLOAT128X: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
    pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
    pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
    pub const __ldiv_t_defined: u32 = 1;
    pub const __lldiv_t_defined: u32 = 1;
    pub const RAND_MAX: u32 = 2147483647;
    pub const EXIT_FAILURE: u32 = 1;
    pub const EXIT_SUCCESS: u32 = 0;
    pub const _MEMORY_H: u32 = 1;
    pub const _STRING_H: u32 = 1;
    pub const _ASSERT_H: u32 = 1;
    pub const sz_VOID: u32 = 0;
    pub const sz_WORD: u32 = 4;
    pub const kindof_VOID: u32 = 0;
    pub const kindof_INT: u32 = 1;
    pub const kindof_FLOAT: u32 = 2;
    pub const kindof_COMPLEX: u32 = 3;
    pub const kindof_VEC3: u32 = 4;
    pub const kindof_VEC4: u32 = 5;
    pub const SILENCE: f64 = 0.0;
    pub const TRUE: u32 = 1;
    pub const FALSE: u32 = 0;
    pub const ONE_PI: f64 = 3.141592653589793;
    pub const TWO_PI: f64 = 6.283185307179586;
    pub const SQRT2: f64 = 1.4142135623730951;
    pub type wchar_t = ::std::os::raw::c_int;
    pub type _Float32 = f32;
    pub type _Float64 = f64;
    pub type _Float32x = f64;
    pub type _Float64x = f64;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct div_t {
        pub quot: ::std::os::raw::c_int,
        pub rem: ::std::os::raw::c_int,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ldiv_t {
        pub quot: ::std::os::raw::c_long,
        pub rem: ::std::os::raw::c_long,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lldiv_t {
        pub quot: ::std::os::raw::c_longlong,
        pub rem: ::std::os::raw::c_longlong,
    }
    extern "C" {
        pub fn __ctype_get_mb_cur_max() -> usize;
    }
    extern "C" {
        pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
    }
    extern "C" {
        pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtod(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
        ) -> f64;
    }
    extern "C" {
        pub fn strtof(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
        ) -> f32;
    }
    extern "C" {
        pub fn strtold(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
        ) -> f64;
    }
    extern "C" {
        pub fn strtol(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn strtoul(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strtoll(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtoull(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub fn rand() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn srand(__seed: ::std::os::raw::c_uint);
    }
    extern "C" {
        pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn realloc(
            __ptr: *mut ::std::os::raw::c_void,
            __size: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn free(__ptr: *mut ::std::os::raw::c_void);
    }
    extern "C" {
        pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn abort();
    }
    extern "C" {
        pub fn atexit(
            __func: ::std::option::Option<unsafe extern "C" fn()>,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn at_quick_exit(
            __func: ::std::option::Option<unsafe extern "C" fn()>,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn exit(__status: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn quick_exit(__status: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn _Exit(__status: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    pub type __compar_fn_t = ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >;
    extern "C" {
        pub fn bsearch(
            __key: *const ::std::os::raw::c_void,
            __base: *const ::std::os::raw::c_void,
            __nmemb: usize,
            __size: usize,
            __compar: root::__compar_fn_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn qsort(
            __base: *mut ::std::os::raw::c_void,
            __nmemb: usize,
            __size: usize,
            __compar: root::__compar_fn_t,
        );
    }
    extern "C" {
        pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> root::div_t;
    }
    extern "C" {
        pub fn ldiv(
            __numer: ::std::os::raw::c_long,
            __denom: ::std::os::raw::c_long,
        ) -> root::ldiv_t;
    }
    extern "C" {
        pub fn lldiv(
            __numer: ::std::os::raw::c_longlong,
            __denom: ::std::os::raw::c_longlong,
        ) -> root::lldiv_t;
    }
    extern "C" {
        pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mbtowc(
            __pwc: *mut root::wchar_t,
            __s: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wctomb(
            __s: *mut ::std::os::raw::c_char,
            __wchar: root::wchar_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mbstowcs(
            __pwcs: *mut root::wchar_t,
            __s: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn wcstombs(
            __s: *mut ::std::os::raw::c_char,
            __pwcs: *const root::wchar_t,
            __n: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn memcpy(
            __dest: *mut ::std::os::raw::c_void,
            __src: *const ::std::os::raw::c_void,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memmove(
            __dest: *mut ::std::os::raw::c_void,
            __src: *const ::std::os::raw::c_void,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memset(
            __s: *mut ::std::os::raw::c_void,
            __c: ::std::os::raw::c_int,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memcmp(
            __s1: *const ::std::os::raw::c_void,
            __s2: *const ::std::os::raw::c_void,
            __n: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn memchr(
            __s: *const ::std::os::raw::c_void,
            __c: ::std::os::raw::c_int,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn strcpy(
            __dest: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strncpy(
            __dest: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strcat(
            __dest: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strncat(
            __dest: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strcmp(
            __s1: *const ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strncmp(
            __s1: *const ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strcoll(
            __s1: *const ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strxfrm(
            __dest: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strchr(
            __s: *const ::std::os::raw::c_char,
            __c: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strrchr(
            __s: *const ::std::os::raw::c_char,
            __c: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strcspn(
            __s: *const ::std::os::raw::c_char,
            __reject: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strspn(
            __s: *const ::std::os::raw::c_char,
            __accept: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strpbrk(
            __s: *const ::std::os::raw::c_char,
            __accept: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strstr(
            __haystack: *const ::std::os::raw::c_char,
            __needle: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strtok(
            __s: *mut ::std::os::raw::c_char,
            __delim: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn __strtok_r(
            __s: *mut ::std::os::raw::c_char,
            __delim: *const ::std::os::raw::c_char,
            __save_ptr: *mut *mut ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::std::os::raw::c_char,
            __file: *const ::std::os::raw::c_char,
            __line: ::std::os::raw::c_uint,
            __function: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn __assert_perror_fail(
            __errnum: ::std::os::raw::c_int,
            __file: *const ::std::os::raw::c_char,
            __line: ::std::os::raw::c_uint,
            __function: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn __assert(
            __assertion: *const ::std::os::raw::c_char,
            __file: *const ::std::os::raw::c_char,
            __line: ::std::os::raw::c_int,
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct t_CKCOMPLEX {
        pub re: f64,
        pub im: f64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct t_CKPOLAR {
        pub modulus: f64,
        pub phase: f64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct t_CKVEC3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct t_CKVEC4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct t_CKVECTOR {
        pub N: ::std::os::raw::c_ulong,
        pub values: *mut f64,
    }
    pub type c_str = *mut ::std::os::raw::c_char;
    pub type c_constr = *const ::std::os::raw::c_char;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct t_CKCOMPLEX_SAMPLE {
        pub re: f64,
        pub im: f64,
    }
    extern "C" {
        #[link_name = "\u{1}ahh_data"]
        pub static mut ahh_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}britestk_data"]
        pub static mut britestk_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}dope_data"]
        pub static mut dope_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}eee_data"]
        pub static mut eee_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}fwavblnk_data"]
        pub static mut fwavblnk_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}halfwave_data"]
        pub static mut halfwave_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}impuls10_data"]
        pub static mut impuls10_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}impuls20_data"]
        pub static mut impuls20_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}impuls40_data"]
        pub static mut impuls40_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}mand1_data"]
        pub static mut mand1_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}mandpluk_data"]
        pub static mut mandpluk_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}marmstk1_data"]
        pub static mut marmstk1_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}ooo_data"]
        pub static mut ooo_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}peksblnk_data"]
        pub static mut peksblnk_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}ppksblnk_data"]
        pub static mut ppksblnk_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}silence_data"]
        pub static mut silence_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}sineblnk_data"]
        pub static mut sineblnk_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}sinewave_data"]
        pub static mut sinewave_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}snglpeak_data"]
        pub static mut snglpeak_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}twopeaks_data"]
        pub static mut twopeaks_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}glot_ahh_data"]
        pub static mut glot_ahh_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}glot_eee_data"]
        pub static mut glot_eee_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}glot_ooo_data"]
        pub static mut glot_ooo_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}glot_pop_data"]
        pub static mut glot_pop_data: [f64; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}ahh_size"]
        pub static mut ahh_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}britestk_size"]
        pub static mut britestk_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}dope_size"]
        pub static mut dope_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}eee_size"]
        pub static mut eee_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}fwavblnk_size"]
        pub static mut fwavblnk_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}halfwave_size"]
        pub static mut halfwave_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}impuls10_size"]
        pub static mut impuls10_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}impuls20_size"]
        pub static mut impuls20_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}impuls40_size"]
        pub static mut impuls40_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}mand1_size"]
        pub static mut mand1_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}mandpluk_size"]
        pub static mut mandpluk_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}marmstk1_size"]
        pub static mut marmstk1_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}ooo_size"]
        pub static mut ooo_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}peksblnk_size"]
        pub static mut peksblnk_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}ppksblnk_size"]
        pub static mut ppksblnk_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}silence_size"]
        pub static mut silence_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}sineblnk_size"]
        pub static mut sineblnk_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}sinewave_size"]
        pub static mut sinewave_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}snglpeak_size"]
        pub static mut snglpeak_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}twopeaks_size"]
        pub static mut twopeaks_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}glot_ahh_size"]
        pub static mut glot_ahh_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}glot_eee_size"]
        pub static mut glot_eee_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}glot_ooo_size"]
        pub static mut glot_ooo_size: ::std::os::raw::c_ulong;
    }
    extern "C" {
        #[link_name = "\u{1}glot_pop_size"]
        pub static mut glot_pop_size: ::std::os::raw::c_ulong;
    }
}