use super::define_vm_types;

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