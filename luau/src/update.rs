use crate::define_vm_types;

pub const SHUFFLES: &'static str = r#"
#define LUAVM_SHUFFLE_COMMA ,
#define LUAVM_SHUFFLE_OTHER ;

#define LUAVM_SHUFFLE3(sep, a1, a2, a3) a2 sep a1 sep a3
#define LUAVM_SHUFFLE4(sep, a1, a2, a3, a4) a2 sep a1 sep a4 sep a3
#define LUAVM_SHUFFLE5(sep, a1, a2, a3, a4, a5) a2 sep a4 sep a1 sep a3 sep a5
#define LUAVM_SHUFFLE6(sep, a1, a2, a3, a4, a5, a6) a6 sep a5 sep a1 sep a2 sep a3 sep a4
#define LUAVM_SHUFFLE7(sep, a1, a2, a3, a4, a5, a6, a7) a7 sep a2 sep a6 sep a3 sep a5 sep a4 sep a1
#define LUAVM_SHUFFLE8(sep, a1, a2, a3, a4, a5, a6, a7, a8) a6 sep a2 sep a1 sep a7 sep a3 sep a8 sep a4 sep a5
#define LUAVM_SHUFFLE9(sep, a1, a2, a3, a4, a5, a6, a7, a8, a9) a2 sep a3 sep a4 sep a1 sep a9 sep a8 sep a5 sep a7 sep a6
"#;

define_vm_types! {
    #define PROTO_MEMBER2_ENC vmvalue4
    #define CLOSURE_DEBUGNAME_ENC vmvalue4
    #define TSTRING_HASH_ENC vmvalue4

    #define PROTO_DEBUGNAME_ENC vmvalue1

    #define CLOSURE_CONT_ENC vmvalue3
    #define PROTO_DEBUGISN_ENC vmvalue3
    #define LSTATE_STACKSIZE_ENC vmvalue3

    #define PROTO_TYPEINFO_ENC vmvalue2
    #define UDATA_META_ENC vmvalue2

    #define PROTO_MEMBER1_ENC vmvalue0    // (removed)
    #define CLOSURF_FUNC_ENC vmvalue0     // (removed)
    #define TABLE_META_ENC vmvalue0       // (removed)
    #define TABLE_MEMBER_ENC vmvalue0     // (removed)
    #define LSTATE_GLOBAL_ENC vmvalue0    // (removed)
    #define TSTRING_LEN_ENC vmvalue0      // (removed)
    #define GSTATE_TTNAME_ENC vmvalue0    // (removed)
    #define GSTATE_TMNAME_ENC vmvalue0    // (removed)
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