use std::{ffi::CString, ops::Deref};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Instance(pub *const usize);
impl Deref for Instance {
    type Target = *const usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Instance {
    pub unsafe fn from_raw(base: *const usize) -> Self {
        Instance(base)
    }

    pub fn parent(&self) -> Instance {
        unsafe {
            *(self.wrapping_byte_add(Self::PARENT) as *const Instance)
        }
    }

    pub fn children(&self) -> InstanceIterator {
        let children_ptr = self.wrapping_byte_add(Self::CHILDREN);
        let end = unsafe { *(children_ptr.wrapping_byte_add(std::mem::size_of::<usize>()) as *const usize) } as usize;

        InstanceIterator {
            base: children_ptr,
            count: 0,
            end,
        }
    }

    pub fn class_name(&self) -> String {
        let c_string = unsafe {
            let class_name = self.wrapping_byte_add(Self::CLASS_DESCRIPTOR + Self::CLASS_NAME);

            // longer strings have `class_name` point instead to another memory address where the string actually lives
            let stored_here: i32 = *(class_name.wrapping_byte_add(Self::CLASS_DESCRIPTOR) as *const i32);
            let class_name_ptr = if stored_here >= 16 {
                *(class_name as *const *const CString)
            } else {
                class_name as *const CString
            };

            (*class_name_ptr).clone()
        };
        c_string.to_string_lossy().to_string()
    }

    pub fn find_first_child_of_class(&self, class_name: &str) -> Option<Instance> {
        self.children().find(|x| x.class_name() == class_name)
    } 
}

pub struct InstanceIterator {
    base: *const usize,
    count: usize,
    end: usize,
}
impl Iterator for InstanceIterator {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.end {
            return None;
        }

        self.count += std::mem::size_of::<usize>() as usize * 2;
        let child_ptr = self.base.wrapping_byte_add(self.count);
        Some(Instance(child_ptr))
    }
}
