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