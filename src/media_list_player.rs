use vlc_sys as sys;

use crate::{Instance, MediaList, MediaPlayer, PlaybackMode};

pub struct MediaListPlayer {
    pub(crate) ptr: *mut sys::libvlc_media_list_player_t,
}

unsafe impl Send for MediaListPlayer {}

impl MediaListPlayer {
    /// Create a new MediaListPlayer instance
    pub fn new(instance: &Instance) -> Option<MediaListPlayer> {
        unsafe {
            let p = sys::libvlc_media_list_player_new(instance.ptr);

            if p.is_null() {
                return None;
            }

            Some(MediaListPlayer { ptr: p })
        }
    }

    /// Get the current MediaPlayer instance used by the [MediaListPlayer]
    pub fn get_media_player(&self) -> Option<MediaPlayer> {
        unsafe {
            let p = sys::libvlc_media_list_player_get_media_player(self.ptr);

            if p.is_null() {
                return None;
            }

            Some(MediaPlayer { ptr: p })
        }
    }

    /// Set the [MediaList] to play using this player
    pub fn set_media_list(&self, media_list: &MediaList) {
        unsafe { sys::libvlc_media_list_player_set_media_list(self.ptr, media_list.ptr) }
    }

    /// Start playback of current [MediaList]
    pub fn play(&self) {
        unsafe { sys::libvlc_media_list_player_play(self.ptr) }
    }

    /// Pause playback of current [MediaList]
    pub fn set_pause(&self, pause: bool) {
        unsafe { sys::libvlc_media_list_player_set_pause(self.ptr, pause as i32) }
    }

    /// Whether the list is currently playing
    pub fn is_playing(&self) -> bool {
        unsafe {
            match sys::libvlc_media_list_player_is_playing(self.ptr) {
                1 => true,
                _ => false,
            }
        }
    }

    /// Play item at given index
    pub fn play_item_at_index(&self, index: i32) -> Result<(), ()> {
        unsafe {
            match sys::libvlc_media_list_player_play_item_at_index(self.ptr, index) {
                0 => Ok(()),
                _ => Err(()),
            }
        }
    }

    /// Stop media playback
    pub fn stop(&self) {
        unsafe { sys::libvlc_media_list_player_stop(self.ptr) }
    }

    /// Skip to next media in list
    pub fn next(&self) -> Result<(), ()> {
        unsafe {
            match sys::libvlc_media_list_player_next(self.ptr) {
                0 => Ok(()),
                _ => Err(()),
            }
        }
    }

    /// Skip to previous media in list
    pub fn previous(&self) -> Result<(), ()> {
        unsafe {
            match sys::libvlc_media_list_player_previous(self.ptr) {
                0 => Ok(()),
                _ => Err(()),
            }
        }
    }

    /// Set the playback mode
    pub fn set_playback_mode(&self, mode: PlaybackMode) {
        unsafe { sys::libvlc_media_list_player_set_playback_mode(self.ptr, mode as u32) }
    }
}

impl Drop for MediaListPlayer {
    fn drop(&mut self) {
        unsafe { sys::libvlc_media_list_player_release(self.ptr) }
    }
}
