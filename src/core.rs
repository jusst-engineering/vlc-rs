// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use crate::enums::*;
use crate::tools::{from_cstr, from_cstr_ref, to_cstr};
use libc::{c_char, c_int, c_void};
use std::borrow::Cow;
use std::convert::TryInto;
use std::ffi::CString;
use std::i32;
use std::marker::PhantomData;
use std::ptr;
use vlc_sys as sys;

/// Retrieve libvlc version.
pub fn version() -> String {
    unsafe {
        from_cstr_ref(sys::libvlc_get_version())
            .unwrap()
            .into_owned()
    }
}

/// Retrieve libvlc compiler version.
pub fn compiler() -> String {
    unsafe {
        from_cstr_ref(sys::libvlc_get_compiler())
            .unwrap()
            .into_owned()
    }
}

pub struct Instance {
    pub(crate) ptr: *mut sys::libvlc_instance_t,
}

unsafe impl Send for Instance {}

impl Instance {
    /// Create and initialize a libvlc instance with specified args.
    /// Note: args.len() has to be less or equal to i32::MAX
    /// Note: libvlc discourages using arguments as these are not guaranteed to be stable between different versions of libvlc
    pub fn with_args(args: Option<Vec<String>>) -> Option<Instance> {
        let args_c_ptr: Vec<*const c_char>;
        let args_c: Vec<CString>;
        if let Some(argv) = args {
            args_c = argv
                .into_iter()
                .map(|x| CString::new(x).expect("Error: Unexpected null byte"))
                .collect();
            args_c_ptr = args_c.iter().map(|x| x.as_ptr()).collect();
        } else {
            args_c_ptr = Vec::new();
        }

        unsafe {
            let p = if args_c_ptr.is_empty() {
                sys::libvlc_new(0, ptr::null())
            } else {
                sys::libvlc_new(args_c_ptr.len() as i32, args_c_ptr.as_ptr())
            };

            if p.is_null() {
                return None;
            }

            Some(Instance { ptr: p })
        }
    }

    /// Create and initialize a libvlc instance.
    pub fn new() -> Option<Instance> {
        Instance::with_args(None)
    }

    /// Sets the application name.
    /// LibVLC passes this as the user agent string when a protocol requires it.
    pub fn set_user_agent(&self, name: &str, http: &str) {
        unsafe {
            sys::libvlc_set_user_agent(self.ptr, to_cstr(name).as_ptr(), to_cstr(http).as_ptr());
        }
    }

    /// Sets some meta-information about the application.
    pub fn set_app_id(&self, id: &str, version: &str, icon: &str) {
        unsafe {
            sys::libvlc_set_app_id(
                self.ptr,
                to_cstr(id).as_ptr(),
                to_cstr(version).as_ptr(),
                to_cstr(icon).as_ptr(),
            );
        }
    }

    /// Returns a list of audio filters that are available.
    pub fn audio_filter_list_get(&self) -> Option<ModuleDescriptionList> {
        unsafe {
            let p = sys::libvlc_audio_filter_list_get(self.ptr);
            if p.is_null() {
                None
            } else {
                Some(ModuleDescriptionList { ptr: p })
            }
        }
    }

    /// Returns a list of video filters that are available.
    pub fn video_filter_list_get(&self) -> Option<ModuleDescriptionList> {
        unsafe {
            let p = sys::libvlc_video_filter_list_get(self.ptr);
            if p.is_null() {
                None
            } else {
                Some(ModuleDescriptionList { ptr: p })
            }
        }
    }

    /// Set logging callback
    pub fn set_log<F: Fn(LogLevel, Log, Cow<str>) + Send + 'static>(&self, f: F) {
        let cb: Box<Box<dyn Fn(LogLevel, Log, Cow<str>) + Send + 'static>> = Box::new(Box::new(f));

        unsafe {
            sys::libvlc_log_set(self.ptr, Some(logging_cb), Box::into_raw(cb) as *mut _);
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_instance_t {
        self.ptr
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            sys::libvlc_release(self.ptr);
        }
    }
}

const BUF_SIZE: usize = 1024; // Write log message to the buffer by vsnprintf.
unsafe extern "C" fn logging_cb(
    data: *mut c_void,
    level: c_int,
    ctx: *const sys::libvlc_log_t,
    fmt: *const c_char,
    args: *mut sys::__va_list_tag,
) {
    let f: &Box<dyn Fn(LogLevel, Log, Cow<str>) + Send + 'static> = ::std::mem::transmute(data);
    let mut buf: [c_char; BUF_SIZE] = [0; BUF_SIZE];

    sys::vsnprintf(buf.as_mut_ptr(), BUF_SIZE.try_into().unwrap(), fmt, args);

    f(
        (level as u32).into(),
        Log { ptr: ctx },
        from_cstr_ref(buf.as_ptr()).unwrap(),
    );
}

/// List of module description.
pub struct ModuleDescriptionList {
    ptr: *mut sys::libvlc_module_description_t,
}

impl ModuleDescriptionList {
    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_module_description_t {
        self.ptr
    }
}

impl Drop for ModuleDescriptionList {
    fn drop(&mut self) {
        unsafe { sys::libvlc_module_description_list_release(self.ptr) };
    }
}

impl<'a> IntoIterator for &'a ModuleDescriptionList {
    type Item = ModuleDescriptionRef<'a>;
    type IntoIter = ModuleDescriptionListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ModuleDescriptionListIter {
            ptr: self.ptr,
            _phantomdata: PhantomData,
        }
    }
}

pub struct ModuleDescriptionListIter<'a> {
    ptr: *mut sys::libvlc_module_description_t,
    _phantomdata: PhantomData<&'a sys::libvlc_module_description_t>,
}

/// Description of a module.
/// The strings are owned.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModuleDescription {
    pub name: Option<String>,
    pub shortname: Option<String>,
    pub longname: Option<String>,
    pub help: Option<String>,
}

/// Description of a module.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModuleDescriptionRef<'a> {
    pub name: Option<Cow<'a, str>>,
    pub shortname: Option<Cow<'a, str>>,
    pub longname: Option<Cow<'a, str>>,
    pub help: Option<Cow<'a, str>>,
}

impl<'a> Iterator for ModuleDescriptionListIter<'a> {
    type Item = ModuleDescriptionRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.ptr.is_null() {
                return None;
            }
            let p = self.ptr;
            self.ptr = (*p).p_next;
            Some(ModuleDescriptionRef {
                name: from_cstr_ref((*p).psz_name),
                shortname: from_cstr_ref((*p).psz_shortname),
                longname: from_cstr_ref((*p).psz_longname),
                help: from_cstr_ref((*p).psz_help),
            })
        }
    }
}

impl<'a> ModuleDescriptionRef<'a> {
    /// Convert to owned strings.
    pub fn into_owned(&'a self) -> ModuleDescription {
        ModuleDescription {
            name: self.name.as_ref().map(|s| s.clone().into_owned()),
            shortname: self.shortname.as_ref().map(|s| s.clone().into_owned()),
            longname: self.name.as_ref().map(|s| s.clone().into_owned()),
            help: self.shortname.as_ref().map(|s| s.clone().into_owned()),
        }
    }
}

pub fn errmsg() -> Option<String> {
    unsafe { from_cstr(sys::libvlc_errmsg()) }
}

pub fn clearerr() {
    unsafe { sys::libvlc_clearerr() };
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct EsChanged {
    i_type: i32,
    psz_id: Option<String>,
}

impl EsChanged {
    unsafe fn from_event(pe: *const sys::libvlc_event_t) -> Self {
        unsafe {
            EsChanged {
                i_type: (*pe).u.media_player_es_changed.i_type,
                psz_id: from_cstr((*pe).u.media_player_es_changed.psz_id),
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TitleDescription {
    duration: i64,
    psz_name: Option<String>,
    flags: u32,
    index: i32,
}

impl TitleDescription {
    unsafe fn from_event(pe: *const sys::libvlc_event_t) -> Self {
        TitleDescription {
            duration: (*(*pe).u.media_player_title_selection_changed.title).i_duration,
            psz_name: from_cstr((*(*pe).u.media_player_title_selection_changed.title).psz_name),
            flags: (*(*pe).u.media_player_title_selection_changed.title).i_flags,
            index: (*pe).u.media_player_title_selection_changed.index,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    MediaMetaChanged(Meta),
    MediaSubItemAdded,
    MediaDurationChanged(i64),
    MediaParsedChanged(i32),
    MediaFreed,
    MediaStateChanged(State),
    MediaSubItemTreeAdded,

    MediaPlayerMediaChanged,
    MediaPlayerNothingSpecial,
    MediaPlayerOpening,
    MediaPlayerBuffering(f32),
    MediaPlayerPlaying,
    MediaPlayerPaused,
    MediaPlayerStopped,
    MediaPlayerForward,
    MediaPlayerBackward,
    MediaPlayerEndReached,
    MediaPlayerEncounteredError,
    MediaPlayerTimeChanged,
    MediaPlayerPositionChanged(f64),
    MediaPlayerSeekableChanged,
    MediaPlayerPausableChanged,
    MediaPlayerSnapshotTaken,
    MediaPlayerLengthChanged,
    MediaPlayerVout,
    //MediaPlayerTeletextChanged(i32),
    MediaListItemDeleted,
    MediaListWillDeleteItem,

    MediaListViewItemAdded,
    MediaListViewWillAddItem,
    MediaListViewItemDeleted,
    MediaListViewWillDeleteItem,

    MediaListPlayerPlayed,
    MediaListPlayerNextItemSet,
    MediaListPlayerStopped,

    MediaDiscovererStarted,
    MediaDiscovererEnded,

    MediaThumbnailGenerated,
    MediaAttachedThumbnailsFound,
    MediaPlayerStopping,
    MediaPlayerESAdded(EsChanged),
    MediaPlayerESDeleted(EsChanged),
    MediaPlayerESSelected {
        i_type: i32,
        psz_selected_id: Option<String>,
        psz_unselected_id: Option<String>,
    },
    MediaPlayerCorked,
    MediaPlayerUncorked,
    MediaPlayerMuted,
    MediaPlayerUnmuted,
    MediaPlayerAudioVolume(f32),
    MediaPlayerAudioDevice(Option<String>),
    MediaPlayerESUpdated(EsChanged),
    MediaPlayerProgramAdded(i32),
    MediaPlayerProgramDeleted(i32),
    MediaPlayerProgramSelected(i32),
    MediaPlayerProgramUpdated(i32),
    MediaPlayerTitleListChanged,
    MediaPlayerTitleSelectionChanged(TitleDescription),
    MediaPlayerChapterChanged(i32),
    MediaPlayerRecordChanged,
    MediaPlayerTeletextPageChanged(i32),
    MediaPlayerTeletextActivePageChanged(i32),
    MediaListItemAdded,
    MediaListWillAddItem,
    MediaListEndReached,
    RendererDiscovererItemAdded,
    RendererDiscovererItemDeleted,
    MediaPlayerMediaStopping,
}

pub struct EventManager<'a> {
    pub(crate) ptr: *mut sys::libvlc_event_manager_t,
    pub(crate) _phantomdata: ::std::marker::PhantomData<&'a sys::libvlc_event_manager_t>,
}

impl<'a> EventManager<'a> {
    pub fn detach(&self, event_type: EventType, registered_callback: *mut c_void) {
        unsafe {
            sys::libvlc_event_detach(
                self.ptr,
                event_type as i32,
                Some(event_manager_callback),
                registered_callback,
            )
        }
    }

    pub fn attach<F>(&self, event_type: EventType, callback: F) -> Result<*mut c_void, ()>
    where
        F: Fn(Event, VLCObject) + Send + 'static,
    {
        // Explicit type annotation is needed
        let callback: Box<Box<dyn Fn(Event, VLCObject) + Send + 'static>> =
            Box::new(Box::new(callback));

        let raw = Box::into_raw(callback) as *mut c_void;

        let result = unsafe {
            sys::libvlc_event_attach(
                self.ptr,
                event_type as i32,
                Some(event_manager_callback),
                raw,
            )
        };

        if result == 0 {
            Ok(raw)
        } else {
            Err(())
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_event_manager_t {
        self.ptr
    }
}

unsafe extern "C" fn event_manager_callback(pe: *const sys::libvlc_event_t, data: *mut c_void) {
    let f: &Box<dyn Fn(Event, VLCObject) + Send + 'static> = ::std::mem::transmute(data);

    f(conv_event(pe), VLCObject { ptr: (*pe).p_obj });
}

// Convert c-style libvlc_event_t to Event
fn conv_event(pe: *const sys::libvlc_event_t) -> Event {
    let event_type: EventType = (unsafe { (*pe).type_ } as u32).into();

    match event_type {
        EventType::MediaMetaChanged => unsafe {
            Event::MediaMetaChanged((*pe).u.media_meta_changed.meta_type.into())
        },
        EventType::MediaSubItemAdded => Event::MediaSubItemAdded,
        EventType::MediaDurationChanged => unsafe {
            Event::MediaDurationChanged((*pe).u.media_duration_changed.new_duration)
        },
        EventType::MediaParsedChanged => unsafe {
            Event::MediaParsedChanged((*pe).u.media_parsed_changed.new_status)
        },
        EventType::MediaSubItemTreeAdded => Event::MediaSubItemTreeAdded,
        EventType::MediaPlayerMediaChanged => Event::MediaPlayerMediaChanged,
        EventType::MediaPlayerNothingSpecial => Event::MediaPlayerNothingSpecial,
        EventType::MediaPlayerOpening => Event::MediaPlayerOpening,
        EventType::MediaPlayerBuffering => unsafe {
            Event::MediaPlayerBuffering((*pe).u.media_player_buffering.new_cache)
        },
        EventType::MediaPlayerPlaying => Event::MediaPlayerPlaying,
        EventType::MediaPlayerPaused => Event::MediaPlayerPaused,
        EventType::MediaPlayerStopped => Event::MediaPlayerStopped,
        EventType::MediaPlayerForward => Event::MediaPlayerForward,
        EventType::MediaPlayerBackward => Event::MediaPlayerBackward,
        EventType::MediaPlayerEncounteredError => Event::MediaPlayerEncounteredError,
        EventType::MediaPlayerTimeChanged => Event::MediaPlayerTimeChanged,
        EventType::MediaPlayerPositionChanged => unsafe {
            Event::MediaPlayerPositionChanged((*pe).u.media_player_position_changed.new_position)
        },
        EventType::MediaPlayerSeekableChanged => Event::MediaPlayerSeekableChanged,
        EventType::MediaPlayerPausableChanged => Event::MediaPlayerPausableChanged,
        EventType::MediaPlayerSnapshotTaken => Event::MediaPlayerSnapshotTaken,
        EventType::MediaPlayerLengthChanged => Event::MediaPlayerLengthChanged,
        EventType::MediaPlayerVout => Event::MediaPlayerVout,
        EventType::MediaPlayerTeletextPageChanged => unsafe {
            Event::MediaPlayerTeletextPageChanged(
                (*pe).u.media_player_teletext_page_changed.page,
            )
        },
        EventType::MediaPlayerTeletextActivePageChanged => unsafe {
            Event::MediaPlayerTeletextActivePageChanged(
                (*pe).u.media_player_teletext_page_changed.page,
            )
        },
        EventType::MediaListItemDeleted => Event::MediaListItemDeleted,
        EventType::MediaListWillDeleteItem => Event::MediaListWillDeleteItem,
        EventType::MediaListViewItemAdded => Event::MediaListViewItemAdded,
        EventType::MediaListViewWillAddItem => Event::MediaListViewWillAddItem,
        EventType::MediaListViewItemDeleted => Event::MediaListViewItemDeleted,
        EventType::MediaListViewWillDeleteItem => Event::MediaListViewWillDeleteItem,
        EventType::MediaListPlayerPlayed => Event::MediaListPlayerPlayed,
        EventType::MediaListPlayerNextItemSet => Event::MediaListPlayerNextItemSet,
        EventType::MediaListPlayerStopped => Event::MediaListPlayerStopped,
        EventType::MediaThumbnailGenerated => Event::MediaThumbnailGenerated,
        EventType::MediaAttachedThumbnailsFound => Event::MediaAttachedThumbnailsFound,
        EventType::MediaPlayerStopping => Event::MediaPlayerStopping,
        EventType::MediaPlayerESAdded => unsafe {
            Event::MediaPlayerESAdded(EsChanged::from_event(pe))
        },
        EventType::MediaPlayerESDeleted => unsafe {
            Event::MediaPlayerESDeleted(EsChanged::from_event(pe))
        },
        EventType::MediaPlayerESSelected => unsafe {
            Event::MediaPlayerESSelected {
                i_type: (*pe).u.media_player_es_selection_changed.i_type,
                psz_selected_id: from_cstr(
                    (*pe).u.media_player_es_selection_changed.psz_selected_id,
                ),
                psz_unselected_id: from_cstr(
                    (*pe).u.media_player_es_selection_changed.psz_unselected_id,
                ),
            }
        },
        EventType::MediaPlayerCorked => Event::MediaPlayerCorked,
        EventType::MediaPlayerUncorked => Event::MediaPlayerUncorked,
        EventType::MediaPlayerMuted => Event::MediaPlayerMuted,
        EventType::MediaPlayerUnmuted => Event::MediaPlayerUnmuted,
        EventType::MediaPlayerAudioVolume => unsafe {
            Event::MediaPlayerAudioVolume((*pe).u.media_player_audio_volume.volume)
        },
        EventType::MediaPlayerAudioDevice => unsafe {
            Event::MediaPlayerAudioDevice(from_cstr((*pe).u.media_player_audio_device.device))
        },
        EventType::MediaPlayerESUpdated => unsafe {
            Event::MediaPlayerESUpdated(EsChanged::from_event(pe))
        },
        EventType::MediaPlayerProgramAdded => unsafe {
            Event::MediaPlayerProgramAdded((*pe).u.media_player_program_changed.i_id)
        },
        EventType::MediaPlayerProgramDeleted => unsafe {
            Event::MediaPlayerProgramDeleted((*pe).u.media_player_program_changed.i_id)
        },
        EventType::MediaPlayerProgramSelected => unsafe {
            Event::MediaPlayerProgramSelected((*pe).u.media_player_program_changed.i_id)
        },
        EventType::MediaPlayerProgramUpdated => unsafe {
            Event::MediaPlayerProgramUpdated((*pe).u.media_player_program_changed.i_id)
        },
        EventType::MediaPlayerTitleListChanged => Event::MediaPlayerTitleListChanged,
        EventType::MediaPlayerTitleSelectionChanged => unsafe {
            Event::MediaPlayerTitleSelectionChanged(TitleDescription::from_event(pe))
        },
        EventType::MediaPlayerChapterChanged => unsafe {
            Event::MediaPlayerChapterChanged((*pe).u.media_player_chapter_changed.new_chapter)
        },
        EventType::MediaPlayerRecordChanged => Event::MediaPlayerRecordChanged,
        EventType::MediaListItemAdded => Event::MediaListItemAdded,
        EventType::MediaListWillAddItem => Event::MediaListWillAddItem,
        EventType::MediaListEndReached => Event::MediaListEndReached,
        EventType::RendererDiscovererItemAdded => Event::RendererDiscovererItemAdded,
        EventType::RendererDiscovererItemDeleted => Event::RendererDiscovererItemDeleted,
        EventType::MediaPlayerMediaStopping => Event::MediaPlayerMediaStopping,
    }
}

pub struct VLCObject {
    ptr: *mut c_void,
}

impl VLCObject {
    /// Returns raw pointer
    pub fn raw(&self) -> *mut c_void {
        self.ptr
    }
}

pub struct Log {
    pub(crate) ptr: *const sys::libvlc_log_t,
}

impl Log {
    /// Returns raw pointer
    pub fn raw(&self) -> *const sys::libvlc_log_t {
        self.ptr
    }
}
