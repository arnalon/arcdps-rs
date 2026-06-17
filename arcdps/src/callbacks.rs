//! Callback types.

use crate::{
    evtc::{Agent, Event},
    imgui,
    util::abi,
};
use num_enum::{FromPrimitive, IntoPrimitive};
use std::ffi::c_char;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Exported struct for ArcDPS plugins.
#[repr(C)]
pub struct ArcDpsExport {
    /// Size of exports table.
    pub size: usize,

    /// Unique plugin signature.
    ///
    /// Pick a random number that is not used by other modules.
    pub sig: u32,

    /// ImGui version number.
    ///
    /// Set to `18000` if you do not use ImGui (as of 2021-02-02).
    pub imgui_version: u32,

    /// Plugin name string.
    pub out_name: *const c_char,

    /// Plugin build (version) string.
    pub out_build: *const c_char,

    /// WndProc callback.
    ///
    /// Return is assigned to uMsg (return zero to not be processed by ArcDPS or game).
    pub wnd_nofilter: Option<RawWndProcCallback>,

    /// Combat callback.
    ///
    /// May be called asynchronously, use `id` to keep track of order.
    /// First event id will be `2`.
    /// Return is ignored.
    pub combat: Option<RawCombatCallback>,

    /// ImGui callback.
    pub imgui: Option<RawImguiCallback>,

    /// Options callback.
    ///
    /// For a plugin options tab.
    pub options_end: Option<RawOptionsCallback>,

    /// Local combat callback.
    ///
    /// Like `combat` (area) but from chat log.
    pub combat_local: Option<RawCombatCallback>,

    /// Filtered WndProc callback.
    ///
    /// Like `wnd_nofilter` but input fitlered using modifiers.
    pub wnd_filter: Option<RawWndProcCallback>,

    /// Options windows callback.
    ///
    /// Called once per window option checkbox in settings, with null at the end.
    /// Non-zero return disables ArcDPS drawing that checkbox.
    pub options_windows: Option<RawOptionsWindowsCallback>,
}

unsafe impl Sync for ArcDpsExport {}

pub type InitFunc = fn() -> Result<(), Option<String>>;

pub type ReleaseFunc = fn();

pub type ReleaseRequestFunc = fn(reason: ExtensionLoad) -> bool;

pub type UpdateUrlFunc = fn() -> Option<String>;

pub type WndProcCallback = fn(key: usize, key_down: bool, prev_key_down: bool) -> bool;

pub type CombatCallback = fn(
    event: Option<&Event>,
    src: Option<&Agent>,
    dst: Option<&Agent>,
    skill_name: Option<&'static str>,
    id: u64,
    revision: u64,
);

pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);

pub type OptionsCallback = fn(ui: &imgui::Ui);

pub type OptionsWindowsCallback = fn(ui: &imgui::Ui, window_name: Option<&str>) -> bool;

abi! {
    pub type RawWndProcCallback =
        unsafe extern fn(h_wnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;

    pub type RawCombatCallback = unsafe extern fn(
        event: *const Event,
        src: *const Agent,
        dst: *const Agent,
        skill_name: *const c_char,
        id: u64,
        revision: u64,
    );

    pub type RawImguiCallback = unsafe extern fn(not_character_select_or_loading: u32);

    pub type RawOptionsCallback = unsafe extern fn();

    pub type RawOptionsWindowsCallback = unsafe extern fn(window_name: *const c_char) -> bool;
}

/// Result of an exentsion load or reason for an extension unload.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u32)]
pub enum ExtensionLoad {
    /// Extension was loaded successfully.
    Ok = 0,

    /// No signature.
    NoSignature = 1,

    /// ImGui version did not match.
    ///
    /// Extension will stay loaded with ImGui callbacks disabled.
    InvalidImGui = 2,

    /// Obsolete ArcDPS module.
    Obsolete = 3,

    /// An extension with the same signature already exists.
    AlreadyLoaded = 4,

    /// Extension did not provide callback function table.
    NoFunctionTableReturned = 5,

    /// Extension did not provide an `init` function.
    NoInitFunctionReturned = 6,

    /// Failed to load extension module with `LoadLibrary`.
    ///
    /// Safe to call `GetLastError`.
    LoadLibaryError = 7,

    /// No slots left.
    NoSlotsLeft = 8,

    /// Extension is missing `get_release_addr` export.
    MissingGetReleaseAddr = 9,

    /// Game shutdown.
    Shutdown = 10,

    /// Removed via call to `freeextension`.
    RemoveViaExport = 11,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u32),
}
