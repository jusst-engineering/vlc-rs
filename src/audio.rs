// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use crate::tools::from_cstr;
use crate::MediaPlayer;
use crate::TrackDescription;
use vlc_sys as sys;

pub trait MediaPlayerAudioEx {
    fn get_mute(&self) -> Option<bool>;
    fn set_mute(&self, muted: bool);
    fn get_volume(&self) -> i32;
    fn set_volume(&self, volume: i32) -> Result<(), ()>;
    fn get_audio_track_description(&self) -> Option<Vec<TrackDescription>>;
    fn get_audio_track(&self) -> Option<i32>;
    fn set_audio_track(&self, track: i32);
}

impl MediaPlayerAudioEx for MediaPlayer {
    fn get_mute(&self) -> Option<bool> {
        let r = unsafe { sys::libvlc_audio_get_mute(self.ptr) };

        if r == 0 {
            Some(false)
        } else if r == -1 {
            None
        } else {
            Some(true)
        }
    }

    fn set_mute(&self, status: bool) {
        unsafe { sys::libvlc_audio_set_mute(self.ptr, if status { 1 } else { 0 }) };
    }

    fn get_volume(&self) -> i32 {
        unsafe { sys::libvlc_audio_get_volume(self.ptr) }
    }
    fn set_volume(&self, volume: i32) -> Result<(), ()> {
        unsafe {
            if sys::libvlc_audio_set_volume(self.ptr, volume) == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }
    fn get_audio_track_description(&self) -> Option<Vec<TrackDescription>> {
        unsafe {
            let p0 = sys::libvlc_audio_get_track_description(self.ptr);
            if p0.is_null() {
                return None;
            }
            let mut td = Vec::new();
            let mut p = p0;

            while !p.is_null() {
                td.push(TrackDescription {
                    id: (*p).i_id,
                    name: from_cstr((*p).psz_name),
                });

                p = (*p).p_next;
            }
            sys::libvlc_track_description_list_release(p0);
            Some(td)
        }
    }
    fn get_audio_track(&self) -> Option<i32> {
        unsafe {
            let track = sys::libvlc_audio_get_track(self.ptr);
            if track == -1 {
                None
            } else {
                Some(track)
            }
        }
    }
    fn set_audio_track(&self, track: i32) {
        unsafe {
            sys::libvlc_audio_set_track(self.ptr, track);
        }
    }
}
