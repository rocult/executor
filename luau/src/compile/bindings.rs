use cxx::CxxString;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CompileOptions {
    pub optimizationLevel: ::std::os::raw::c_int,
    pub debugLevel: ::std::os::raw::c_int,
    pub typeInfoLevel: ::std::os::raw::c_int,
    pub coverageLevel: ::std::os::raw::c_int,
    pub vectorLib: *const ::std::os::raw::c_char,
    pub vectorCtor: *const ::std::os::raw::c_char,
    pub vectorType: *const ::std::os::raw::c_char,
    pub mutableGlobals: *const *const ::std::os::raw::c_char,
    pub userdataTypes: *const *const ::std::os::raw::c_char,
    pub librariesWithKnownMembers: *const *const ::std::os::raw::c_char,
    pub libraryMemberTypeCb: LibraryMemberTypeCallback,
    pub libraryMemberConstantCb: LibraryMemberConstantCallback,
    pub disabledBuiltins: *const *const ::std::os::raw::c_char,
}
impl Default for CompileOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type CompileConstant = *mut ::std::os::raw::c_void;
pub type LibraryMemberTypeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        library: *const ::std::os::raw::c_char,
        member: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type LibraryMemberConstantCallback = ::std::option::Option<
    unsafe extern "C" fn(
        library: *const ::std::os::raw::c_char,
        member: *const ::std::os::raw::c_char,
        constant: *mut CompileConstant,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BytecodeEncoder {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct optional {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ParseOptions {
    pub allowDeclarationSyntax: bool,
    pub captureComments: bool,
    pub parseFragment: optional,
    pub storeCstData: bool,
    pub noErrorLimit: bool,
}
impl Default for ParseOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

unsafe extern "C" {
    #[link_name = "\u{1}?compile@Luau@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBV23@AEBUCompileOptions@1@AEBUParseOptions@1@PEAVBytecodeEncoder@1@@Z"]
    pub fn compile(
        source: *const i8,
        options: *const CompileOptions,
        parseOptions: *const ParseOptions,
        encoder: *mut BytecodeEncoder,
    ) -> CxxString;
}