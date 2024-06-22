use std::hash::{Hash, Hasher};
use std::ptr;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, Default)]
enum InnerRc<T> {
    #[default]
    None,
    Strong(Rc<T>),
    Weak(Weak<T>),
}

impl <T> InnerRc<T> {
    pub fn present(&self) -> bool {
        match self {
            InnerRc::None => false,
            InnerRc::Strong(_) => true,
            InnerRc::Weak(rc) => rc.strong_count() > 0,
        }
    }

    pub fn get(&self) -> Option<&T> {
        match self {
            InnerRc::None => None,
            InnerRc::Strong(rc) => Some(rc.as_ref()),
            InnerRc::Weak(rc) => if rc.strong_count() > 0 {
                Some(unsafe { &*rc.as_ptr() })
            } else {
                None
            }
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            InnerRc::None => None,
            InnerRc::Strong(rc) => Rc::get_mut(rc),
            InnerRc::Weak(rc) => if rc.strong_count() > 0 {
                Some(unsafe { &mut *(rc.as_ptr() as *mut T) })
            } else {
                None
            }
        }
    }
}

impl <T> PartialEq for InnerRc<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            InnerRc::None => match other {
                InnerRc::None => true,
                _ => false
            }
            InnerRc::Strong(rc1) => match other {
                InnerRc::None => false,
                InnerRc::Strong(rc2) => Rc::ptr_eq(rc1, rc2),
                InnerRc::Weak(rc2) => ptr::addr_eq(rc1.as_ref() as *const T, rc2.as_ptr())
            }
            InnerRc::Weak(rc1) => match other {
                InnerRc::None => false,
                InnerRc::Strong(rc2) => ptr::addr_eq(rc1.as_ptr(), rc2.as_ref() as *const T),
                InnerRc::Weak(rc2) => ptr::addr_eq(rc1.as_ptr(), rc2.as_ptr())
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Handle<T> {
    id: usize,
    data: InnerRc<T>,
    dirty: bool
}

impl <T> Eq for Handle<T> {}

impl <T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl <T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl <T> Handle<T> {
    pub const EMPTY: Self = Self {
        id: 0,
        data: InnerRc::None,
        dirty: false,
    };

    pub fn new_strong(data: T) -> Self {
        let rc = Rc::new(data);
        Self {
            id: Rc::as_ptr(&rc) as usize,
            data: InnerRc::Strong(rc),
            dirty: true
        }
    }

    pub fn clone_weak(&self) -> Self {
        Self {
            id: self.id,
            data: match &self.data {
                InnerRc::None => InnerRc::None,
                InnerRc::Strong(rc) => InnerRc::Weak(Rc::downgrade(&rc)),
                InnerRc::Weak(rc) => InnerRc::Weak(rc.clone()),
            },
            dirty: true,
        }
    }

    pub fn clone_strong(&self) -> Self {
        let data = match &self.data {
            InnerRc::None => InnerRc::None,
            InnerRc::Strong(rc) => InnerRc::Strong(rc.clone()),
            InnerRc::Weak(rc) => if let Some(rc) = rc.upgrade() {
                InnerRc::Strong(rc)
            } else {
                InnerRc::None
            },
        };
        Self {
            id: self.id,
            dirty: data != InnerRc::None,
            data,
        }
    }

    pub fn from_rc(data: &Rc<T>) -> Self {
        Self {
            id: Rc::as_ptr(data) as usize,
            data: InnerRc::Weak(Rc::downgrade(data)),
            dirty: true
        }
    }

    pub fn present(&self) -> bool {
        self.data.present()
    }

    pub fn get(&self) -> Option<&T> {
        self.data.get()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.present() {
            self.dirty = true;
        }
        self.data.get_mut()
    }

    pub fn dirty(&self) -> bool {
        self.present() && self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}