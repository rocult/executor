use std::marker::PhantomData;

pub type Vm0<T> = T;
pub type Vm1<T> = VMValue<T, Add, Sub, Left, Left>;
pub type Vm2<T> = VMValue<T, Sub, Add, Left, Left>;
pub type Vm3<T> = VMValue<T, Sub, Sub, Right, Right>;
pub type Vm4<T> = VMValue<T, Xor, Xor, Right, Right>;

macro_rules! define_vm_types {
    (vmvalue0) => { $crate::Vm0<T> };
    (vmvalue1) => { $crate::Vm1<T> };
    (vmvalue2) => { $crate::Vm2<T> };
    (vmvalue3) => { $crate::Vm3<T> };
    (vmvalue4) => { $crate::Vm4<T> };
    ($(#define $name:ident $vm_value:ident)*) => {
        $(
            #[allow(non_camel_case_types)]
            pub type $name<T> = define_vm_types!($vm_value);
        )*
    };
    ($($name:ident = $vm_type:ident);*;) => {
        pub fn type_to_vm(ident: &str, ty: &str) -> Option<syn::Type> {
            match ident {
                $(
                    stringify!($name) => syn::parse_str::<syn::Type>(&format!("{}<{}>", stringify!($vm_type), ty)).ok(),
                )*
                _ => None
            }
        }
    };
}
pub(crate) use define_vm_types;

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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[repr(C)]
pub struct VMValue<T, GetOp, SetOp, GetDir, SetDir> {
    storage: T,
    _phantom: PhantomData<(GetOp, SetOp, GetDir, SetDir)>,
}

impl<T, GetOp, SetOp, GetDir, SetDir> VMValue<T, GetOp, SetOp, GetDir, SetDir> {
    pub fn new(storage: T) -> Self {
        VMValue {
            storage,
            _phantom: PhantomData,
        }
    }

    fn calculate_ptr(base: usize, offset: usize, op: Operation, dir: Direction) -> usize {
        match (op, dir) {
            (Operation::Add, Direction::Right) | (Operation::Add, Direction::Left) => base + offset,
            (Operation::Sub, Direction::Right) | (Operation::Sub, Direction::Left) => base - offset,
            (Operation::Xor, Direction::Right) | (Operation::Xor, Direction::Left) => base ^ offset,
        }
    }
}

impl<T, GetOp, SetOp, GetDir, SetDir> VMValue<T, GetOp, SetOp, GetDir, SetDir>
where
    T: Copy,
    GetOp: OperationTrait,
    GetDir: DirectionTrait,
{
    pub fn get(&self) -> T {
        let base = self as *const _ as usize;
        let offset = &self.storage as *const _ as usize;
        let result_ptr = Self::calculate_ptr(base, offset, GetOp::operation(), GetDir::direction());
        unsafe { std::ptr::read(result_ptr as *const _) }
    }
}

impl<T, GetOp, SetOp, GetDir, SetDir> VMValue<T, GetOp, SetOp, GetDir, SetDir>
where
    T: Copy,
    SetOp: OperationTrait,
    SetDir: DirectionTrait,
{
    pub fn set(&mut self, value: T)
    where
        T: Copy,
        SetOp: OperationTrait,
        SetDir: DirectionTrait,
    {
        let base = self as *const _ as usize;
        let input_value = &value as *const _ as usize;
        let result_ptr =
            Self::calculate_ptr(base, input_value, SetOp::operation(), SetDir::direction());
        self.storage = unsafe { std::ptr::read(result_ptr as *const _) };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Operation {
    Add,
    Sub,
    Xor,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub trait OperationTrait {
    fn operation() -> Operation;
}

pub trait DirectionTrait {
    fn direction() -> Direction;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Add;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Sub;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Xor;

impl OperationTrait for Add {
    fn operation() -> Operation {
        Operation::Add
    }
}

impl OperationTrait for Sub {
    fn operation() -> Operation {
        Operation::Sub
    }
}

impl OperationTrait for Xor {
    fn operation() -> Operation {
        Operation::Xor
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Left;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Right;

impl DirectionTrait for Left {
    fn direction() -> Direction {
        Direction::Left
    }
}

impl DirectionTrait for Right {
    fn direction() -> Direction {
        Direction::Right
    }
}
