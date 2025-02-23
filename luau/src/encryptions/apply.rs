use std::marker::PhantomData;

pub type VmAddSub<T> = VMValue<T, Add, Sub, Left, Left>;
pub type VmSubAdd<T> = VMValue<T, Sub, Add, Left, Left>;
pub type VmSubSub<T> = VMValue<T, Sub, Sub, Right, Right>;
pub type VmXorXor<T> = VMValue<T, Xor, Xor, Right, Right>;

macro_rules! define_vm_types {
    ($($name:ident<$t:ident> = $vm_type:ident<$t2:ident>);*;) => {
        $(
            #[allow(non_camel_case_types)]
            pub type $name<$t> = $vm_type<$t2>;
        )*

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

define_vm_types! {
    global_Statettname<T> = VmXorXor<T>;
    global_Statetmname<T> = VmXorXor<T>;

    lua_Stateglobal<T> = VmXorXor<T>;
    lua_Statestacksize<T> = VmXorXor<T>;

    TStringhash<T> = VmSubAdd<T>;
    TStringlen<T> = VmXorXor<T>;

    Udatametatable<T> = VmXorXor<T>;

    Closure__bindgen_ty_1__bindgen_ty_1f<T> = VmAddSub<T>;
    Closure__bindgen_ty_1__bindgen_ty_1cont<T> = VmSubSub<T>;
    Closure__bindgen_ty_1__bindgen_ty_1debugname<T> = VmSubAdd<T>;

    Closure__bindgen_ty_1__bindgen_ty_2p<T> = VmAddSub<T>;

    Protok<T> = VmAddSub<T>;
    Protocode<T> = VmAddSub<T>;
    Protop<T> = VmAddSub<T>;

    Protolineinfo<T> = VmSubAdd<T>;
    Protoabslineinfo<T> = VmSubAdd<T>;
    Protolocvars<T> = VmSubAdd<T>;
    Protoupvalues<T> = VmSubAdd<T>;
    Protosource<T> = VmSubAdd<T>;

    Protodebugname<T> = VmAddSub<T>;
    Protodebuginsn<T> = VmSubSub<T>;

    Prototypeinfo<T> = VmXorXor<T>;

    LuaTablemetatable<T> = VmAddSub<T>;
    LuaTablearray<T> = VmAddSub<T>;
    LuaTablenode<T> = VmAddSub<T>;
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
        let result_ptr = Self::calculate_ptr(base, input_value, SetOp::operation(), SetDir::direction());
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