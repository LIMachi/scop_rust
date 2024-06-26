use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::{Rc, Weak};

pub trait Get<T> {
    fn get(&self) -> &T;
}

pub trait GetMut<T> {
    fn get_mut(&mut self) -> &mut T;
}

pub trait Set<T> {
    fn set(&mut self, value: T);
}

pub trait SetGlobal<T, G> {
    fn set(&self, global: &mut G, value: T);
}

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

#[derive(Debug, Default, Clone)]
pub struct Handle<T> {
    id: usize,
    rc: InnerRc<(bool, T)>
}

impl <T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.rc.get().map(|r| &r.1).unwrap()
    }
}

impl <T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rc.get_mut().map(|r| { r.0 = true; &mut r.1 }).unwrap()
    }
}

impl <T> Eq for Handle<T> {}

impl <T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl <T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write_usize(self.id); }
}

impl <T> Handle<T> {
    pub const EMPTY: Self = Self { id: 0, rc: InnerRc::None };

    pub fn get<V>(&self) -> &V where T: Get<V> {
        self.rc.get().map(|r| r.1.get()).unwrap()
    }
    
    pub fn get_mut<V>(&mut self) -> &mut V where T: GetMut<V> {
        self.rc.get_mut().map(|r| {r.0 = true; r.1.get_mut() }).unwrap()
    }

    pub fn set<V>(&mut self, value: V) where T: Set<V> {
        if let Some((b, v)) = self.rc.get_mut() {
            *b = true;
            v.set(value);
        }
    }

    pub fn new_strong(data: T) -> Self {
        let rc = Rc::new((false, data));
        Self {
            id: Rc::as_ptr(&rc) as usize,
            rc: InnerRc::Strong(rc),
        }
    }

    pub fn clone_weak(&self) -> Self {
        Self {
            id: self.id,
            rc: match &self.rc {
                InnerRc::None => InnerRc::None,
                InnerRc::Strong(rc) => InnerRc::Weak(Rc::downgrade(&rc)),
                InnerRc::Weak(rc) => InnerRc::Weak(rc.clone()),
            }
        }
    }

    pub fn clone_strong(&self) -> Self {
        Self {
            id: self.id,
            rc: match &self.rc {
                InnerRc::None => InnerRc::None,
                InnerRc::Strong(rc) => InnerRc::Strong(rc.clone()),
                InnerRc::Weak(rc) => if let Some(rc) = rc.upgrade() {
                    InnerRc::Strong(rc)
                } else {
                    InnerRc::None
                },
            }
        }
    }

    pub fn present(&self) -> bool { self.rc.present() }

    pub fn dirty(&self) -> bool { self.rc.get().map(|(d, _)| *d).unwrap_or(false) }
    
    pub fn manual_dirty(&mut self) {
        if let Some((d, _)) = self.rc.get_mut() {
            *d = true;
        }
    }

    pub fn clear_dirty(&mut self) -> bool {
        if let Some((d, _)) = self.rc.get_mut() {
            let was_dirty = *d;
            *d = false;
            was_dirty
        } else {
            false
        }
    }
    
    pub fn id(&self) -> usize { self.id }
}