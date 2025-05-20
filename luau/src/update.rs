use crate::define_vm_types;

pub mod funcs {
    pub const LUAA_TOOBJECT: usize = 0x25A0540;
    pub const LUAC_STEP: usize = 0x25B08D0;
    pub const LUAD_THROW: usize = 0x25B0DE0;
    pub const LUAL_CHECKLSTRING: usize = 0x25A3ED0;
    pub const LUAL_GETMETAFIELD: usize = 0x25A43D0;
    pub const LUAL_REGISTER: usize = 0x25A5D80;
    pub const LUAM_VISITGCO: usize = 0x25F93B0;
    pub const LUAO_PUSHVFSTRING: usize = 0x260E9F0;
    pub const LUAU_LOAD: usize = 0xAD32E0;
    pub const LUAU_EXECUTE: usize = 0x25E3B50;
}

pub const SHUFFLES: &'static str = r#"
#define LUAU_SHUFFLE_COMMA ,
#define LUAU_SHUFFLE_OTHER ;

#define LUAU_SHUFFLE3(s, a1, a2, a3) a1 s a2 s a3
#define LUAU_SHUFFLE4(s, a1, a2, a3, a4) a3 s a2 s a1 s a4
#define LUAU_SHUFFLE5(s, a1, a2, a3, a4, a5) a1 s a5 s a2 s a3 s a4
#define LUAU_SHUFFLE6(s, a1, a2, a3, a4, a5, a6) a2 s a1 s a4 s a6 s a5 s a3
#define LUAU_SHUFFLE7(s, a1, a2, a3, a4, a5, a6, a7) a4 s a1 s a3 s a5 s a7 s a2 s a6
#define LUAU_SHUFFLE8(s, a1, a2, a3, a4, a5, a6, a7, a8) a6 s a2 s a1 s a7 s a3 s a8 s a4 s a5
#define LUAU_SHUFFLE9(s, a1, a2, a3, a4, a5, a6, a7, a8, a9) a1 s a5 s a3 s a2 s a4 s a9 s a8 s a7 s a6
"#;

define_vm_types! {
    #define PROTO_MEMBER1_ENC vmvalue0
    #define PROTO_MEMBER2_ENC vmvalue2
    #define PROTO_DEBUGISN_ENC vmvalue4
    #define PROTO_TYPEINFO_ENC vmvalue1
    #define PROTO_DEBUGNAME_ENC vmvalue3

    #define LSTATE_STACKSIZE_ENC vmvalue4
    #define LSTATE_GLOBAL_ENC vmvalue0

    #define CLOSURE_FUNC_ENC vmvalue0
    #define CLOSURF_FUNC_ENC vmvalue0
    #define CLOSURE_CONT_ENC vmvalue4
    #define CLOSURE_DEBUGNAME_ENC vmvalue2

    #define TABLE_MEMBER_ENC vmvalue0
    #define TABLE_META_ENC vmvalue0

    #define UDATA_META_ENC vmvalue1

    #define TSTRING_HASH_ENC vmvalue2
    #define TSTRING_LEN_ENC vmvalue0

    #define GSTATE_TTNAME_ENC vmvalue0
    #define GSTATE_TMNAME_ENC vmvalue0
}

define_vm_types! {
    // GSTATE_*_ENC
    global_Statettname = GSTATE_TTNAME_ENC;
    global_Statetmname = GSTATE_TMNAME_ENC;

    // LSTATE_*_ENC
    lua_Stateglobal = LSTATE_GLOBAL_ENC;
    lua_Statestacksize = LSTATE_STACKSIZE_ENC;

    // TSTRING_*_ENC
    TStringhash = TSTRING_HASH_ENC;
    TStringlen = TSTRING_LEN_ENC;

    // UDATA_META_ENC
    Udatametatable = UDATA_META_ENC;

    // CLOSURE_*_ENC
    Closure__bindgen_ty_1__bindgen_ty_1cont = CLOSURE_CONT_ENC;
    Closure__bindgen_ty_1__bindgen_ty_1debugname = CLOSURE_DEBUGNAME_ENC;

    // CLOSURE_FUNC_ENC
    Closure__bindgen_ty_1__bindgen_ty_1f = CLOSURF_FUNC_ENC;
    Closure__bindgen_ty_1__bindgen_ty_2p = CLOSURF_FUNC_ENC;

    // PROTO_MEMBER1_ENC
    Protok = PROTO_MEMBER1_ENC;
    Protocode = PROTO_MEMBER1_ENC;
    Protop = PROTO_MEMBER1_ENC;

    // PROTO_MEMBER2_ENC
    Protolineinfo = PROTO_MEMBER2_ENC;
    Protoabslineinfo = PROTO_MEMBER2_ENC;
    Protolocvars = PROTO_MEMBER2_ENC;
    Protoupvalues = PROTO_MEMBER2_ENC;
    Protosource = PROTO_MEMBER2_ENC;

    // PROTO_*_ENC
    Protodebugname = PROTO_DEBUGNAME_ENC;
    Protodebuginsn = PROTO_DEBUGISN_ENC;

    // PROTO_TYPEINFO_ENC
    Prototypeinfo = PROTO_TYPEINFO_ENC;

    // TABLE_MEMBER_ENC, TABLE_META_ENC
    LuaTablearray = TABLE_MEMBER_ENC;
    LuaTablenode = TABLE_MEMBER_ENC;
    LuaTablemetatable = TABLE_META_ENC;
}
