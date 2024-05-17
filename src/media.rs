// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use crate::enums::{Meta, TrackType};
use crate::tools::{from_cstr, path_to_cstr, to_cstr};
use crate::{EventManager, Instance};
use std::path::Path;
use vlc_sys as sys;

pub struct Media {
    pub(crate) ptr: *mut sys::libvlc_media_t,
}

unsafe impl Send for Media {}

impl Media {
    /// Create a media with a certain given media resource location, for instance a valid URL.
    pub fn new_location(mrl: &str) -> Option<Media> {
        let cstr = to_cstr(mrl);

        unsafe {
            let p = sys::libvlc_media_new_location(cstr.as_ptr());
            if p.is_null() {
                return None;
            }

            Some(Media { ptr: p })
        }
    }

    /// Create a media for a certain file path.
    pub fn new_path<T: AsRef<Path>>(path: T) -> Option<Media> {
        let cstr = match path_to_cstr(path.as_ref()) {
            Ok(s) => s,
            Err(_) => {
                return None;
            }
        };

        unsafe {
            let p = sys::libvlc_media_new_path(cstr.as_ptr());
            if p.is_null() {
                return None;
            }

            Some(Media { ptr: p })
        }
    }

    pub fn new_fd(fd: i32) -> Option<Media> {
        unsafe {
            let p = sys::libvlc_media_new_fd(fd);
            if p.is_null() {
                return None;
            }

            Some(Media { ptr: p })
        }
    }

    pub fn mrl(&self) -> Option<String> {
        unsafe {
            let p_str = sys::libvlc_media_get_mrl(self.ptr);
            let s = from_cstr(p_str);
            sys::libvlc_free(p_str as *mut ::libc::c_void);
            s
        }
    }

    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe {
            let p = sys::libvlc_media_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager {
                ptr: p,
                _phantomdata: ::std::marker::PhantomData,
            }
        }
    }

    /// Read the meta of the media.
    /// If the media has not yet been parsed this will return None.
    pub fn get_meta(&self, meta: Meta) -> Option<String> {
        unsafe {
            let p_str = sys::libvlc_media_get_meta(self.ptr, meta as u32);
            let s = from_cstr(p_str);
            sys::libvlc_free(p_str as *mut ::libc::c_void);
            s
        }
    }

    /// Set the meta of the media.
    /// (This function will not save the meta, call save_meta in order to save the meta)
    pub fn set_meta(&self, meta: Meta, value: &str) {
        unsafe {
            sys::libvlc_media_set_meta(self.ptr, meta as u32, to_cstr(value).as_ptr());
        }
    }

    /// Save the meta previously set.
    pub fn save_meta(&self, instance: &Instance) -> bool {
        if unsafe { sys::libvlc_media_save_meta(instance.ptr, self.ptr) } == 0 {
            false
        } else {
            true
        }
    }

    /// Get duration (in ms) of media descriptor object item.
    pub fn duration(&self) -> Option<i64> {
        let time = unsafe { sys::libvlc_media_get_duration(self.ptr) };
        if time != -1 {
            Some(time)
        } else {
            None
        }
    }

    /// Returns statistics of current media playback
    pub fn stats(&self) -> Option<MediaStats> {
        unsafe {
            let mut p_stats: sys::libvlc_media_stats_t = sys::libvlc_media_stats_t {
                i_read_bytes: 0,
                f_input_bitrate: 0.0,
                i_demux_read_bytes: 0,
                f_demux_bitrate: 0.0,
                i_demux_corrupted: 0,
                i_demux_discontinuity: 0,
                i_decoded_video: 0,
                i_decoded_audio: 0,
                i_displayed_pictures: 0,
                i_lost_pictures: 0,
                i_played_abuffers: 0,
                i_lost_abuffers: 0,
                i_late_pictures: 0,
            };

            let ok = sys::libvlc_media_get_stats(self.ptr, &mut p_stats);
            if !ok {
                return None;
            }

            Some(MediaStats {
                read_bytes: p_stats.i_read_bytes,
                input_bitrate: p_stats.f_input_bitrate,
                demux_read_bytes: p_stats.i_demux_read_bytes,
                demux_bitrate: p_stats.f_demux_bitrate,
                demux_corrupted: p_stats.i_demux_corrupted,
                demux_discontinuity: p_stats.i_demux_discontinuity,
                decoded_video: p_stats.i_decoded_video,
                decoded_audio: p_stats.i_decoded_audio,
                displayed_pictures: p_stats.i_displayed_pictures,
                lost_pictures: p_stats.i_lost_pictures,
                played_abuffers: p_stats.i_played_abuffers,
                lost_abuffers: p_stats.i_lost_abuffers,
                late_pictures: p_stats.i_late_pictures,
            })
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_t {
        self.ptr
    }
}

impl Drop for Media {
    fn drop(&mut self) {
        unsafe { sys::libvlc_media_release(self.ptr) };
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MediaTrack {
    pub codec: u32,
    pub original_fourcc: u32,
    pub id: i32,
    pub track_type: TrackType,
    pub profile: i32,
    pub level: i32,
    pub bitrate: u32,
    pub language: Option<String>,
    pub description: Option<String>,
    pub type_specific_data: MediaTrackUnion,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MediaTrackUnion {
    Audio(AudioTrack),
    Video(VideoTrack),
    Subtitle(SubtitleTrack),
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AudioTrack {
    pub channels: u32,
    pub rate: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VideoTrack {
    pub height: u32,
    pub width: u32,
    pub sar_num: u32,
    pub sar_den: u32,
    pub frame_rate_num: u32,
    pub frame_rate_den: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SubtitleTrack {
    pub encoding: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MediaStats {
    pub read_bytes: u64,
    pub input_bitrate: f32,
    pub demux_read_bytes: u64,
    pub demux_bitrate: f32,
    pub demux_corrupted: u64,
    pub demux_discontinuity: u64,
    pub decoded_video: u64,
    pub decoded_audio: u64,
    pub displayed_pictures: u64,
    pub lost_pictures: u64,
    pub played_abuffers: u64,
    pub lost_abuffers: u64,
    pub late_pictures: u64,
}
