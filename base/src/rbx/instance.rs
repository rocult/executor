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
    pub fn parent(&self) -> Instance {
        unsafe {
            *(self.offset(Self::PARENT) as *const Instance)
        }
    }

    pub fn children(&self) -> InstanceIterator {
        let children_ptr = unsafe { self.offset(Self::CHILDREN) };
        let end = unsafe { *(children_ptr.offset(std::mem::size_of::<usize>() as isize) as *const usize) } as isize;

        InstanceIterator {
            base: children_ptr,
            count: 0,
            end,
        }
    }

    pub fn class_name(&self) -> String {
        let c_string = unsafe {
            let class_name = self.offset(Self::CLASS_DESCRIPTOR + Self::CLASS_NAME);

            // longer strings have `class_name` point instead to another memory address where the string actually lives
            let stored_here: i32 = *(class_name.offset(Self::CLASS_DESCRIPTOR) as *const i32);
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
    count: isize,
    end: isize,
}
impl Iterator for InstanceIterator {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.end {
            return None;
        }

        self.count += std::mem::size_of::<usize>() as isize * 2;
        let child_ptr = unsafe { self.base.offset(self.count) };
        Some(Instance(child_ptr))
    }
}
