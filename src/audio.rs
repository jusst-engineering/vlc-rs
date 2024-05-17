// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use crate::MediaPlayer;
use crate::TrackType;
use vlc_sys as sys;

pub trait MediaPlayerAudioEx {
    fn get_mute(&self) -> Option<bool>;
    fn set_mute(&self, muted: bool);
    fn get_volume(&self) -> i32;
    fn set_volume(&self, volume: i32) -> Result<(), ()>;
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
    fn get_audio_track(&self) -> Option<i32> {
        unsafe {
            let track = sys::libvlc_media_player_get_selected_track(self.ptr, TrackType::Audio as i32);
            if !track.is_null() {
                let i_id =(*track).i_id;
                sys::libvlc_media_track_release(track);
                Some(i_id)
            } else {
                None
            }
        }
    }
    fn set_audio_track(&self, track: i32) {
        unsafe {
            sys::libvlc_media_player_unselect_track_type(self.ptr, TrackType::Audio as i32);
            let tracklist = sys::libvlc_media_player_get_tracklist(self.ptr, TrackType::Audio as i32, false);

            for i in 0..sys::libvlc_media_tracklist_count(tracklist) {
                let p_track = sys::libvlc_media_tracklist_at(tracklist, i);
                if (*p_track).i_id == track {
                    sys::libvlc_media_player_select_track(self.ptr, p_track);
                    break;
                }
            }

            sys::libvlc_media_tracklist_delete(tracklist);
        }
    }
}
