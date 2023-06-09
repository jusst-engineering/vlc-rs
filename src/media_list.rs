// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use vlc_sys as sys;
use crate::{Instance, Media, EventManager};

pub struct MediaList {
    pub(crate) ptr: *mut sys::libvlc_media_list_t,
}

unsafe impl Send for MediaList{}

impl MediaList {
    /// Create an empty media list.
    pub fn new(instance: &Instance) -> Option<MediaList> {
        unsafe{
            let p = sys::libvlc_media_list_new(instance.ptr);
            if p.is_null() { None }else{ Some(MediaList{ptr: p}) }
        }
    }

    /// Associate media instance with this media list instance.
    /// If another media instance was present it will be released. The libvlc_media_list_lock should NOT be held upon entering this function.
    pub fn set_media(&self, md: &Media) {
        unsafe{ sys::libvlc_media_list_set_media(self.ptr, md.ptr); }
    }

    /// Get media instance from this media list instance.
    pub fn media(&self) -> Option<Media> {
        self.lock();
        let result = unsafe{
            let p = sys::libvlc_media_list_media(self.ptr);
            if p.is_null() { None }else{ Some(Media{ptr: p}) }
        };
        self.unlock();

        result
    }

    /// Add media instance to media list.
    pub fn add_media(&self, md: &Media) -> Result<(), ()> {
        self.lock();
        let result = unsafe{
            if sys::libvlc_media_list_add_media(self.ptr, md.ptr) == 0 { Ok(()) }else{ Err(()) }
        };
        self.unlock();

        result
    }

    /// Insert media instance in media list on a position.
    pub fn insert_media(&self, md: &Media, pos: i32) -> Result<(), ()> {
        self.lock();
        let result = unsafe{
            if sys::libvlc_media_list_insert_media(self.ptr, md.ptr, pos) == 0 { Ok(()) }else{ Err(()) }
        };
        self.unlock();

        result
    }

    /// Remove media instance from media list on a position.
    pub fn remove_index(&self, pos: i32) -> Result<(), ()> {
        self.lock();
        let result = unsafe{
            if sys::libvlc_media_list_remove_index(self.ptr, pos) == 0 { Ok(()) }else{ Err(()) }
        };
        self.unlock();

        result

    }

    /// Get count on media list items.
    pub fn count(&self) -> i32 {
        self.lock();
        let result = unsafe{ sys::libvlc_media_list_count(self.ptr) };
        self.unlock();

        result

    }

    /// List media instance in media list at a position.
    pub fn item_at_index(&self, pos: i32) -> Option<Media> {
        self.lock();
        let result = unsafe{
            let p = sys::libvlc_media_list_item_at_index(self.ptr, pos);
            if p.is_null() { None }else{ Some(Media{ptr: p}) }
        };
        self.unlock();

        result

    }

    /// Find index position of List media instance in media list.
    pub fn index_of_item(&self, md: &Media) -> Option<i32> {
        unsafe{
            let i = sys::libvlc_media_list_index_of_item(self.ptr, md.ptr);
            if i == -1 { None }else{ Some(i) }
        }
    }

    /// This indicates if this media list is read-only from a user point of view.
    pub fn is_readonly(&self) -> bool {
        unsafe{ if sys::libvlc_media_list_is_readonly(self.ptr) == 0 { false }else{ true } }
    }

    /// Get lock on media list items
    fn lock(&self) {
        unsafe{ sys::libvlc_media_list_lock(self.ptr); }
    }

    /// Release lock on media list items
    /// The libvlc_media_list_lock should be held upon entering this function.
    fn unlock(&self) {
        unsafe{ sys::libvlc_media_list_unlock(self.ptr); }
    }

    /// Get EventManager from this media list instance.
    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = sys::libvlc_media_list_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_list_t {
        self.ptr
    }
}

impl Drop for MediaList {
    fn drop(&mut self) {
        unsafe{ sys::libvlc_media_list_release(self.ptr) };
    }
}
