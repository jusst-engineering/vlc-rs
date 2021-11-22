// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use libc::c_char;
use std::borrow::Cow;
use std::ffi::{CStr, CString, NulError};
use std::path::Path;

// Convert String to CString.
// Panic if the string includes null bytes.
pub fn to_cstr(s: &str) -> CString {
    CString::new(s.to_owned()).expect("Error: Unexpected null byte")
}

// Convert *const c_char to String
pub unsafe fn from_cstr(p: *const c_char) -> Option<String> {
    if p.is_null() {
        None
    } else {
        let cstr = CStr::from_ptr(p);

        Some(cstr.to_string_lossy().into_owned())
    }
}

// Convert *const c_char to &str
pub unsafe fn from_cstr_ref<'a>(p: *const c_char) -> Option<Cow<'a, str>> {
    if p.is_null() {
        None
    } else {
        let cstr = CStr::from_ptr(p);

        Some(cstr.to_string_lossy())
    }
}

// Create CString from &Path
pub fn path_to_cstr(path: &Path) -> Result<CString, NulError> {
    let path = CString::new(path.to_string_lossy().into_owned())?;

    Ok(path)
}

/// Defines a module with all necessary structures to easily handle a C linked list
macro_rules! linked_list_iter {
    (
      $c_type:ident,
      $name:ident,
      {
        $($(#[$field_meta:meta])*
        $field_vis:vis $field_name:ident: ($field_type:ty, $field_b_type:ty)),* $(,)+
      }
    ) => {
      paste::item! {
        mod $name {
          use std::borrow::Cow;
          use std::marker::PhantomData;
          use crate::tools::from_cstr_ref;

          use vlc_sys::[<$c_type _t>];
          use vlc_sys::[<$c_type _list_release>];
  
          #[derive(Clone, PartialEq, Eq, Hash, Debug)]
          pub struct Item {
            $(
              $(#[$field_meta:meta])*
              $field_vis $field_name : Option<$field_type>,
            )*
          }
  
          #[derive(Clone, PartialEq, Eq, Hash, Debug)]
          pub struct ItemRef<'a> {
            $(
              $(#[$field_meta:meta])*
              $field_vis $field_name : Option<Cow<'a, $field_b_type>>,
            )*
          }
  
          impl<'a> ItemRef<'a> {
            /// Convert to owned strings.
            pub fn into_owned(&'a self) -> Item {
              Item {
                $($field_name: self.$field_name.as_ref().map(|s| s.clone().into_owned()),)*
              }
            }
          }
  
          pub struct ListIter<'a> {
            ptr: *mut [<$c_type _t>],
            _phantomdata: PhantomData<&'a [<$c_type _t>]>,
          }
  
          impl<'a> Iterator for ListIter<'a> {
            type Item = ItemRef<'a>;
  
            fn next(&mut self) -> Option<Self::Item> {
              unsafe {
                if self.ptr.is_null() {
                  return None;
                }
                let p = self.ptr;
                self.ptr = (*p).p_next;
                Some(ItemRef {
                  $($field_name: from_cstr_ref((*p).[<psz_ $field_name>]),)*
                })
              }
            }
          }
  
          pub struct List {
            ptr: *mut [<$c_type _t>]
          }
  
          impl List {
            pub fn new(ptr: *mut [<$c_type _t>]) -> List {
              Self { ptr }
            }
  
            /// Returns raw pointer
            pub fn raw(&self) -> *mut [<$c_type _t>] {
              self.ptr
            }
          }
  
          impl Drop for List {
            fn drop(&mut self) {
              unsafe{ [<$c_type _list_release>](self.ptr) };
            }
          }
  
          impl<'a> IntoIterator for &'a List {
            type Item = ItemRef<'a>;
            type IntoIter = ListIter<'a>;
  
            fn into_iter(self) -> Self::IntoIter {
              ListIter{ptr: self.ptr, _phantomdata: PhantomData}
            }
          }
        }
  
        // backward compatibility types
        pub type [<$name:camel>] = $name::Item;
        pub type [<$name:camel Ref>]<'a> = $name::ItemRef<'a>;
        pub type [<$name:camel List>] = $name::List;
        pub type [<$name:camel ListIter>]<'a> = $name::ListIter<'a>;
      }
    }
  }
  
  /*
  macro_rules! linked_list_iter {
    (
      $namespace:path,
      $c_type: ident,
      $name: ident,
      {
        $($(#[$field_meta:meta])*
        $field_vis:vis $field_name:ident: ($field_type:ty, $field_b_type:ty)),* $(,)+
      }
    ) => {
      paste::item! {
        use std::marker::PhantomData;
        use crate::$namespace::[<$c_type _t>];
        use crate::$namespace::[<$c_type _list_release>];
  
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $name {
          $(
            $(#[$field_meta:meta])*
            $field_vis $field_name : Option<$field_type>,
          )*
        }
  
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct [<$name Ref>]<'a> {
          $(
            $(#[$field_meta:meta])*
            $field_vis $field_name : Option<Cow<'a, $field_b_type>>,
          )*
        }
  
        impl<'a> [<$name Ref>]<'a> {
          /// Convert to owned strings.
          pub fn into_owned(&'a self) -> $name {
            $name {
              $($field_name: self.$field_name.as_ref().map(|s| s.clone().into_owned()),)*
            }
          }
        }
  
        pub struct [<$name ListIter>]<'a> {
          ptr: *mut [<$c_type _t>],
          _phantomdata: PhantomData<&'a [<$c_type _t>]>,
        }
  
        impl<'a> Iterator for [<$name ListIter>]<'a> {
          type Item = [<$name Ref>]<'a>;
  
          fn next(&mut self) -> Option<Self::Item> {
            unsafe {
              if self.ptr.is_null() {
                return None;
              }
              let p = self.ptr;
              self.ptr = (*p).p_next;
              Some([<$name Ref>] {
                $($field_name: from_cstr_ref((*p).[<psz_ $field_name>]),)*
              })
            }
          }
        }
  
        pub struct [<$name List>] {
          ptr: *mut [<$c_type _t>]
        }
  
        impl [<$name List>] {
          /// Returns raw pointer
          pub fn raw(&self) -> *mut [<$c_type _t>] {
            self.ptr
          }
        }
  
        impl Drop for [<$name List>] {
          fn drop(&mut self) {
            unsafe{ [<$c_type _list_release>](self.ptr) };
          }
        }
  
        impl<'a> IntoIterator for &'a [<$name List>] {
          type Item = [<$name Ref>]<'a>;
          type IntoIter = [<$name ListIter>]<'a>;
  
          fn into_iter(self) -> Self::IntoIter {
            [<$name ListIter>]{ptr: self.ptr, _phantomdata: PhantomData}
          }
        }
      }
    }
  }
  */
  
  pub(crate) use linked_list_iter;