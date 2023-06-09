// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use std::time::Duration;

use crate::enums::VideoAdjustOption;
use crate::tools::{from_cstr, to_cstr};
use crate::MediaPlayer;
use crate::TrackDescription;
use libc::c_void;
use vlc_sys as sys;

pub trait MediaPlayerVideoEx {
    fn toggle_fullscreen(&self);
    fn set_fullscreen(&self, fullscreen: bool);
    fn get_fullscreen(&self) -> bool;
    fn set_key_input(&self, on: bool);
    fn set_mouse_input(&self, on: bool);
    fn get_size(&self, num: u32) -> Option<(u32, u32)>;
    fn get_video_track(&self) -> Option<i32>;
    fn set_video_track(&self, track: i32);
    fn get_cursor(&self, num: u32) -> Option<(i32, i32)>;
    fn get_scale(&self) -> f32;
    fn set_scale(&self, factor: f32);
    fn get_aspect_ratio(&self) -> Option<String>;
    fn set_aspect_ratio(&self, aspect: Option<&str>);
    fn get_video_track_description(&self) -> Option<Vec<TrackDescription>>;
    fn get_adjust_int(&self, option: VideoAdjustOption) -> i32;
    fn set_adjust_int(&self, option: VideoAdjustOption, value: i32);
    fn get_adjust_float(&self, option: VideoAdjustOption) -> f32;
    fn set_adjust_float(&self, option: VideoAdjustOption, value: f32);

    fn get_spu_track_description(&self) -> Option<Vec<TrackDescription>>;
    fn get_spu_track(&self) -> Option<i32>;
    fn set_spu_track(&self, track: i32);

    /// Get the requested teletext page.
    fn get_teletext(&self) -> i32;

    /// Get the currently showing teletext page.
    fn get_teletext_active(&self) -> i32;

    /// Set the teletext page to display.
    ///
    /// This function can also be used to send a teletext key.
    ///
    /// # Arguments
    ///
    /// * `page` - If set to 0, teletext is disabled.
    ///            If set between 1 and 999, the according teletext page is loaded.
    ///            Also supports the values of TeletextKey enum for following special links.
    fn set_teletext(&self, page: u32);

    /// Set the teletext background mode.
    ///
    /// # Arguments
    ///
    /// * `opaque` - If `true` an opaque background is rendered behind teletext pages, otherwise
    ///              the background will be transparent.
    fn set_teletext_opaque(&self, opaque: bool);

    /// Configure a message to show as marquee on top of the video.
    ///
    /// # Arguments
    ///
    /// * `message` - If `Some`, the text to display, if `None` the marquee is disabled.
    fn set_marquee(
        &self,
        message: Option<&str>,
        timeout: Option<Duration>,
        x: Option<i32>,
        y: Option<i32>,
    );

    /// Set the marquee text color.
    ///
    /// # Arguments
    ///
    /// * `color` - A RGB value as `(r << 16 | g << 16 | b)` with `r`, `g`, `b` being u8 each.
    fn set_marquee_color(&self, color: i32);

    /// Set the marquee text opacity.
    ///
    /// # Arguments
    ///
    /// * `opacity` - The opacity as u8, 0 being fully transparent, 255 being fully opaque.
    fn set_marquee_opacity(&self, opacity: u8);

    /// Set deinterlacing mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - If `Some`, the name of the deinterlacing mode to load, if `None` deinterlacing
    ///            is disabled.
    ///            Supported modes depend on the vlc configuration.
    fn set_deinterlace(&self, mode: Option<&str>);
}

impl MediaPlayerVideoEx for MediaPlayer {
    fn toggle_fullscreen(&self) {
        unsafe {
            sys::libvlc_toggle_fullscreen(self.ptr);
        }
    }
    fn set_fullscreen(&self, fullscreen: bool) {
        unsafe {
            sys::libvlc_set_fullscreen(self.ptr, if fullscreen { 1 } else { 0 });
        }
    }
    fn get_fullscreen(&self) -> bool {
        unsafe {
            if sys::libvlc_get_fullscreen(self.ptr) == 0 {
                false
            } else {
                true
            }
        }
    }
    fn set_key_input(&self, on: bool) {
        unsafe {
            sys::libvlc_video_set_key_input(self.ptr, if on { 1 } else { 0 });
        }
    }
    fn set_mouse_input(&self, on: bool) {
        unsafe {
            sys::libvlc_video_set_mouse_input(self.ptr, if on { 1 } else { 0 });
        }
    }
    fn get_size(&self, num: u32) -> Option<(u32, u32)> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            let res = sys::libvlc_video_get_size(self.ptr, num, &mut x, &mut y);
            if res == -1 {
                None
            } else {
                Some((x, y))
            }
        }
    }
    fn get_cursor(&self, num: u32) -> Option<(i32, i32)> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            let res = sys::libvlc_video_get_cursor(self.ptr, num, &mut x, &mut y);
            if res == -1 {
                None
            } else {
                Some((x, y))
            }
        }
    }
    fn get_scale(&self) -> f32 {
        unsafe { sys::libvlc_video_get_scale(self.ptr) }
    }
    fn set_scale(&self, factor: f32) {
        unsafe {
            sys::libvlc_video_set_scale(self.ptr, factor);
        }
    }
    fn get_video_track(&self) -> Option<i32> {
        unsafe {
            let track = sys::libvlc_video_get_track(self.ptr);
            if track == -1 {
                None
            } else {
                Some(track)
            }
        }
    }
    fn set_video_track(&self, track: i32) {
        unsafe {
            sys::libvlc_video_set_track(self.ptr, track);
        }
    }
    fn get_aspect_ratio(&self) -> Option<String> {
        unsafe {
            let p = sys::libvlc_video_get_aspect_ratio(self.ptr);
            let s = from_cstr(p);
            if !p.is_null() {
                sys::libvlc_free(p as *mut c_void);
            }
            s
        }
    }
    fn set_aspect_ratio(&self, aspect: Option<&str>) {
        unsafe {
            if let Some(a) = aspect {
                sys::libvlc_video_set_aspect_ratio(self.ptr, to_cstr(a).as_ptr());
            } else {
                sys::libvlc_video_set_aspect_ratio(self.ptr, ::std::ptr::null());
            }
        }
    }
    fn get_video_track_description(&self) -> Option<Vec<TrackDescription>> {
        unsafe {
            let p0 = sys::libvlc_video_get_track_description(self.ptr);
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
    fn get_adjust_int(&self, option: VideoAdjustOption) -> i32 {
        unsafe { sys::libvlc_video_get_adjust_int(self.ptr, option as u32) }
    }
    fn set_adjust_int(&self, option: VideoAdjustOption, value: i32) {
        unsafe {
            sys::libvlc_video_set_adjust_int(self.ptr, option as u32, value);
        }
    }
    fn get_adjust_float(&self, option: VideoAdjustOption) -> f32 {
        unsafe { sys::libvlc_video_get_adjust_float(self.ptr, option as u32) }
    }
    fn set_adjust_float(&self, option: VideoAdjustOption, value: f32) {
        unsafe {
            sys::libvlc_video_set_adjust_float(self.ptr, option as u32, value);
        }
    }

    fn get_spu_track_description(&self) -> Option<Vec<TrackDescription>> {
        unsafe {
            let tracks = sys::libvlc_video_get_spu_description(self.ptr);
            if tracks.is_null() {
                return None;
            }

            let mut track_vector = Vec::new();
            let mut track = tracks;

            while !track.is_null() {
                track_vector.push(TrackDescription {
                    id: (*track).i_id,
                    name: from_cstr((*track).psz_name),
                });

                track = (*track).p_next;
            }

            sys::libvlc_track_description_release(tracks);
            Some(track_vector)
        }
    }

    fn get_spu_track(&self) -> Option<i32> {
        unsafe {
            let track = sys::libvlc_video_get_spu(self.ptr);
            if track == -1 {
                None
            } else {
                Some(track)
            }
        }
    }

    fn set_spu_track(&self, track: i32) {
        unsafe {
            sys::libvlc_video_set_spu(self.ptr, track);
        }
    }

    fn get_teletext(&self) -> i32 {
        unsafe { sys::libvlc_video_get_teletext(self.ptr) }
    }

    fn get_teletext_active(&self) -> i32 {
        unsafe { sys::libvlc_video_get_teletext_active(self.ptr) }
    }

    fn set_teletext(&self, page: u32) {
        unsafe {
            sys::libvlc_video_set_teletext(self.ptr, page as i32);
        }
    }

    fn set_teletext_opaque(&self, opaque: bool) {
        unsafe {
            sys::libvlc_video_set_teletext_opaque(self.ptr, opaque);
        }
    }

    fn set_marquee(
        &self,
        message: Option<&str>,
        timeout: Option<Duration>,
        x: Option<i32>,
        y: Option<i32>,
    ) {
        if let Some(msg) = message {
            unsafe {
                sys::libvlc_video_set_marquee_string(
                    self.ptr,
                    sys::libvlc_video_marquee_option_t_libvlc_marquee_Text,
                    to_cstr(msg).as_ptr(),
                );

                sys::libvlc_video_set_marquee_int(
                    self.ptr,
                    sys::libvlc_video_marquee_option_t_libvlc_marquee_Timeout,
                    timeout
                        .unwrap_or(Duration::ZERO)
                        .as_millis()
                        .clamp(0, i32::MAX as u128) as i32,
                );
                sys::libvlc_video_set_marquee_int(
                    self.ptr,
                    sys::libvlc_video_marquee_option_t_libvlc_marquee_X,
                    x.unwrap_or(0),
                );
                sys::libvlc_video_set_marquee_int(
                    self.ptr,
                    sys::libvlc_video_marquee_option_t_libvlc_marquee_Y,
                    y.unwrap_or(0),
                );
                sys::libvlc_video_set_marquee_int(
                    self.ptr,
                    sys::libvlc_video_marquee_option_t_libvlc_marquee_Enable,
                    1,
                );
            }
        } else {
            unsafe {
                sys::libvlc_video_set_marquee_int(
                    self.ptr,
                    sys::libvlc_video_marquee_option_t_libvlc_marquee_Enable,
                    0,
                );
            }
        }
    }

    fn set_marquee_color(&self, color: i32) {
        unsafe {
            sys::libvlc_video_set_marquee_int(
                self.ptr,
                sys::libvlc_video_marquee_option_t_libvlc_marquee_Color,
                color,
            );
        }
    }

    fn set_marquee_opacity(&self, opacity: u8) {
        unsafe {
            sys::libvlc_video_set_marquee_int(
                self.ptr,
                sys::libvlc_video_marquee_option_t_libvlc_marquee_Opacity,
                opacity as i32,
            );
        }
    }

    fn set_deinterlace(&self, mode: Option<&str>) {
        if let Some(mode) = mode {
            unsafe {
                sys::libvlc_video_set_deinterlace(self.ptr, to_cstr(mode).as_ptr());
            }
        } else {
            unsafe {
                sys::libvlc_video_set_deinterlace(self.ptr, ::std::ptr::null());
            }
        }
    }
}
