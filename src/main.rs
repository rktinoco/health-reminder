#![windows_subsystem = "windows"]

use std::env;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::{null, null_mut};
use std::time::{Duration, Instant};

// Small Win32 declarations keep the release binary dependency-free.
type Handle = *mut c_void;
type Hwnd = Handle;
type Hinstance = Handle;
type Hicon = Handle;
type Hmenu = Handle;
type Wparam = usize;
type Lparam = isize;
type Lresult = isize;

const WM_DESTROY: u32 = 0x0002;
const WM_COMMAND: u32 = 0x0111;
const WM_TIMER: u32 = 0x0113;
const WM_USER: u32 = 0x0400;
const WM_APP_TRAY: u32 = WM_USER + 1;
const WM_RBUTTONUP: usize = 0x0205;
const TIMER_ID: usize = 1;

const IDM_PAUSE: usize = 100;
const IDM_STARTUP: usize = 101;
const IDM_DRINK_15: usize = 110;
const IDM_DRINK_30: usize = 111;
const IDM_DRINK_60: usize = 112;
const IDM_EXERCISE_30: usize = 120;
const IDM_EXERCISE_60: usize = 121;
const IDM_EXERCISE_120: usize = 122;
const IDM_LANG_EN: usize = 130;
const IDM_LANG_ES: usize = 131;
const IDM_LANG_PT: usize = 132;
const IDM_TEST: usize = 140;
const IDM_ABOUT: usize = 198;
const IDM_EXIT: usize = 199;
const IDI_WATER: usize = 3;
const IDI_EXERCISE: usize = 4;

const NIM_ADD: u32 = 0;
const NIM_DELETE: u32 = 2;
const NIM_MODIFY: u32 = 1;
const NIF_MESSAGE: u32 = 1;
const NIF_ICON: u32 = 2;
const NIF_TIP: u32 = 4;
const NIF_INFO: u32 = 16;
const NIIF_INFO: u32 = 1;
const NIIF_USER: u32 = 4;
const MF_STRING: u32 = 0;
const MF_POPUP: u32 = 0x10;
const MF_SEPARATOR: u32 = 0x800;
const MF_CHECKED: u32 = 8;
const MF_UNCHECKED: u32 = 0;
const TPM_BOTTOMALIGN: u32 = 32;
const TPM_LEFTALIGN: u32 = 0;
const TPM_RIGHTBUTTON: u32 = 2;
const KEY_READ: u32 = 0x20019;
const KEY_WRITE: u32 = 0x20006;
const REG_SZ: u32 = 1;
const REG_DWORD: u32 = 4;
const ERROR_SUCCESS: i32 = 0;
const HKEY_CURRENT_USER: Handle = 0x80000001usize as Handle;
const SETTINGS_KEY: &str = "Software\\RKTSI\\HealthReminder";
const IDI_APPLICATION: usize = 32512;
const IDI_HEALTH: usize = 1;
const IDI_HEALTH_PAUSED: usize = 2;
const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: Handle = (-4isize) as Handle;

#[repr(C)]
struct Point { x: i32, y: i32 }

#[repr(C)]
struct WndClassW {
    style: u32,
    lpfn_wnd_proc: WndProc,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: Hinstance,
    h_icon: Hicon,
    h_cursor: Handle,
    hbr_background: Handle,
    lpsz_menu_name: *const u16,
    lpsz_class_name: *const u16,
}

type WndProc = Option<unsafe extern "system" fn(Hwnd, u32, Wparam, Lparam) -> Lresult>;

#[repr(C)]
struct NotifyIconDataW {
    cb_size: u32,
    h_wnd: Hwnd,
    u_id: u32,
    u_flags: u32,
    u_callback_message: u32,
    h_icon: Hicon,
    sz_tip: [u16; 128],
    dw_state: u32,
    dw_state_mask: u32,
    sz_info: [u16; 256],
    u_timeout_or_version: u32,
    sz_info_title: [u16; 64],
    dw_info_flags: u32,
    guid: [u8; 16],
    h_balloon_icon: Hicon,
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassW(class: *const WndClassW) -> u16;
    fn CreateWindowExW(ex_style: u32, class_name: *const u16, window_name: *const u16, style: u32, x: i32, y: i32, width: i32, height: i32, parent: Hwnd, menu: Hmenu, instance: Hinstance, param: *mut c_void) -> Hwnd;
    fn DefWindowProcW(hwnd: Hwnd, message: u32, w_param: Wparam, l_param: Lparam) -> Lresult;
    fn DestroyWindow(hwnd: Hwnd) -> i32;
    fn DispatchMessageW(message: *const Message) -> Lresult;
    fn GetCursorPos(point: *mut Point) -> i32;
    fn GetMessageW(message: *mut Message, hwnd: Hwnd, min: u32, max: u32) -> i32;
    fn GetModuleHandleW(name: *const u16) -> Hinstance;
    fn LoadIconW(instance: Hinstance, name: *const u16) -> Hicon;
    fn SetProcessDpiAwarenessContext(value: Handle) -> i32;
    fn SetProcessDPIAware() -> i32;
    fn PostQuitMessage(exit_code: i32);
    fn SetForegroundWindow(hwnd: Hwnd) -> i32;
    fn SetTimer(hwnd: Hwnd, id: usize, milliseconds: u32, callback: Handle) -> usize;
    fn KillTimer(hwnd: Hwnd, id: usize) -> i32;
    fn SetWindowLongPtrW(hwnd: Hwnd, index: i32, value: isize) -> isize;
    fn GetWindowLongPtrW(hwnd: Hwnd, index: i32) -> isize;
    fn CreatePopupMenu() -> Hmenu;
    fn AppendMenuW(menu: Hmenu, flags: u32, id: usize, text: *const u16) -> i32;
    fn CheckMenuItem(menu: Hmenu, id: usize, check: u32) -> u32;
    fn TrackPopupMenu(menu: Hmenu, flags: u32, x: i32, y: i32, reserved: i32, hwnd: Hwnd, rect: *const c_void) -> i32;
    fn DestroyMenu(menu: Hmenu) -> i32;
    fn MessageBoxW(hwnd: Hwnd, text: *const u16, caption: *const u16, kind: u32) -> i32;
}

#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconW(message: u32, data: *mut NotifyIconDataW) -> i32;
}

#[link(name = "advapi32")]
extern "system" {
    fn RegCreateKeyExW(key: Handle, sub_key: *const u16, reserved: u32, class: *mut u16, options: u32, access: u32, security: *const c_void, result: *mut Handle, disposition: *mut u32) -> i32;
    fn RegOpenKeyExW(key: Handle, sub_key: *const u16, options: u32, access: u32, result: *mut Handle) -> i32;
    fn RegSetValueExW(key: Handle, value_name: *const u16, reserved: u32, kind: u32, data: *const u8, data_size: u32) -> i32;
    fn RegDeleteValueW(key: Handle, value_name: *const u16) -> i32;
    fn RegQueryValueExW(key: Handle, value_name: *const u16, reserved: *mut u32, kind: *mut u32, data: *mut u8, data_size: *mut u32) -> i32;
    fn RegCloseKey(key: Handle) -> i32;
}

#[repr(C)]
struct Message { hwnd: Hwnd, message: u32, w_param: Wparam, l_param: Lparam, time: u32, point: Point }

#[derive(Clone, Copy)]
enum Locale { English, Spanish, Portuguese }

struct Texts {
    name: &'static str,
    drink_title: &'static str,
    drink_body: &'static str,
    exercise_title: &'static str,
    exercise_body: &'static str,
    test_title: &'static str,
    test_body: &'static str,
    paused_tooltip: &'static str,
    next: &'static str,
    water: &'static str,
    movement: &'static str,
    pause: &'static str,
    resume: &'static str,
    startup: &'static str,
    drink: &'static str,
    exercise: &'static str,
    settings: &'static str,
    intervals: &'static str,
    language: &'static str,
    english: &'static str,
    spanish: &'static str,
    portuguese: &'static str,
    about: &'static str,
    about_title: &'static str,
    version_label: &'static str,
    about_attribution: &'static str,
    test: &'static str,
    exit: &'static str,
    error: &'static str,
}

fn texts(locale: Locale) -> Texts {
    match locale {
        Locale::English => Texts { name: "Health Reminder", drink_title: "Time to drink water", drink_body: "Take a short break and drink a glass of water.", exercise_title: "Time to move", exercise_body: "Stand up and stretch for a few minutes.", test_title: "Test notification", test_body: "Health Reminder notifications are working.", paused_tooltip: "Paused", next: "Next", water: "water", movement: "exercise", pause: "Pause reminders", resume: "Resume reminders", startup: "Start with Windows", drink: "Water interval", exercise: "Exercise interval", settings: "Settings", intervals: "Intervals", language: "Language", english: "English", spanish: "Spanish", portuguese: "Portuguese", about: "About", about_title: "About Health Reminder", version_label: "Version", about_attribution: "Developed by Rafael Tinoco using Claude", test: "Test notification", exit: "Exit", error: "Could not update Windows startup settings." },
        Locale::Spanish => Texts { name: "Recordatorio de salud", drink_title: "Hora de beber agua", drink_body: "Haz una pausa y bebe un vaso de agua.", exercise_title: "Hora de moverse", exercise_body: "Levántate y estira durante unos minutos.", test_title: "Notificación de prueba", test_body: "Las notificaciones funcionan correctamente.", paused_tooltip: "Pausado", next: "Próximo", water: "agua", movement: "ejercicio", pause: "Pausar recordatorios", resume: "Continuar recordatorios", startup: "Iniciar con Windows", drink: "Intervalo de agua", exercise: "Intervalo de ejercicio", settings: "Configuración", intervals: "Intervalos", language: "Idioma", english: "Inglés", spanish: "Español", portuguese: "Portugués", about: "Acerca de", about_title: "Acerca de Health Reminder", version_label: "Versión", about_attribution: "Desarrollado por Rafael Tinoco con Claude", test: "Notificación de prueba", exit: "Salir", error: "No se pudieron actualizar las opciones de inicio de Windows." },
        Locale::Portuguese => Texts { name: "Lembrete de saúde", drink_title: "Hora de beber água", drink_body: "Faça uma pausa e beba um copo de água.", exercise_title: "Hora de se movimentar", exercise_body: "Levante-se e alongue-se por alguns minutos.", test_title: "Notificação de teste", test_body: "As notificações estão funcionando.", paused_tooltip: "Pausado", next: "Próxima", water: "água", movement: "exercício", pause: "Pausar lembretes", resume: "Continuar lembretes", startup: "Iniciar com o Windows", drink: "Intervalo de água", exercise: "Intervalo de exercício", settings: "Configurações", intervals: "Intervalos", language: "Idioma", english: "Inglês", spanish: "Espanhol", portuguese: "Português", about: "Sobre", about_title: "Sobre o Health Reminder", version_label: "Versão", about_attribution: "Desenvolvido por Rafael Tinoco usando Claude", test: "Notificação de teste", exit: "Sair", error: "Não foi possível atualizar as configurações de inicialização do Windows." },
    }
}

struct AppState {
    locale: Locale,
    drink_seconds: u64,
    exercise_seconds: u64,
    next_drink: Instant,
    next_exercise: Instant,
    paused: bool,
    startup: bool,
}

fn wide(value: &str) -> Vec<u16> { value.encode_utf16().chain(Some(0)).collect() }
fn now() -> Instant { Instant::now() }
fn checked(state: bool) -> u32 { if state { MF_CHECKED } else { MF_UNCHECKED } }

fn remaining_text(seconds: u64) -> String {
    let minutes = seconds.saturating_add(59) / 60;
    if minutes >= 60 { format!("{}h {}m", minutes / 60, minutes % 60) } else { format!("{}m", minutes) }
}

fn tray_tooltip(state: &AppState) -> String {
    let t = texts(state.locale);
    if state.paused { return format!("{} - {}", t.name, t.paused_tooltip); }
    let (label, due) = if state.next_drink <= state.next_exercise { (t.water, state.next_drink) } else { (t.movement, state.next_exercise) };
    let remaining = due.saturating_duration_since(now()).as_secs();
    format!("{} - {} {} {}", t.name, t.next, label, remaining_text(remaining))
}

fn read_setting(name: &str, default: u32) -> u32 {
    let key_path = wide(SETTINGS_KEY);
    let value_name = wide(name);
    let mut key = null_mut();
    unsafe {
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, KEY_READ, &mut key) != ERROR_SUCCESS { return default; }
        let mut kind = 0;
        let mut value = 0u32;
        let mut size = size_of::<u32>() as u32;
        let result = RegQueryValueExW(key, value_name.as_ptr(), null_mut(), &mut kind, &mut value as *mut u32 as *mut u8, &mut size);
        RegCloseKey(key);
        if result == ERROR_SUCCESS && kind == REG_DWORD && size == size_of::<u32>() as u32 { value } else { default }
    }
}

fn write_setting(name: &str, value: u32) {
    let key_path = wide(SETTINGS_KEY);
    let value_name = wide(name);
    let mut key = null_mut();
    unsafe {
        let mut disposition = 0;
        if RegCreateKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, null_mut(), 0, KEY_WRITE, null(), &mut key, &mut disposition) != ERROR_SUCCESS { return; }
        RegSetValueExW(key, value_name.as_ptr(), 0, REG_DWORD, &value as *const u32 as *const u8, size_of::<u32>() as u32);
        RegCloseKey(key);
    }
}

fn save_settings(state: &AppState) {
    let language = match state.locale { Locale::English => 0, Locale::Spanish => 1, Locale::Portuguese => 2 };
    write_setting("Language", language);
    write_setting("DrinkIntervalSeconds", state.drink_seconds as u32);
    write_setting("ExerciseIntervalSeconds", state.exercise_seconds as u32);
    write_setting("Paused", if state.paused { 1 } else { 0 });
    write_setting("StartupEnabled", if state.startup { 1 } else { 0 });
}

fn valid_drink_interval(value: u32) -> u64 {
    match value { 900 | 1800 | 3600 => value as u64, _ => 1800 }
}

fn valid_exercise_interval(value: u32) -> u64 {
    match value { 1800 | 3600 | 7200 => value as u64, _ => 3600 }
}

fn load_locale() -> Locale {
    let env_locale = match env::var("HEALTH_LANG").ok().as_deref() { Some("es") | Some("ES") => 1, Some("pt") | Some("PT") => 2, _ => 0 };
    match read_setting("Language", env_locale) { 1 => Locale::Spanish, 2 => Locale::Portuguese, _ => Locale::English }
}

fn set_startup(enabled: bool) -> bool {
    let key_path = wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let value_name = wide("HealthReminder");
    let mut key = null_mut();
    unsafe {
        if enabled {
            let mut disposition = 0;
            if RegCreateKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, null_mut(), 0, KEY_WRITE, null(), &mut key, &mut disposition) != ERROR_SUCCESS { return false; }
            let mut exe = [0u16; 32768];
            let length = get_module_filename(&mut exe);
            let command = format!("\"{}\"", String::from_utf16_lossy(&exe[..length]));
            let command_w = wide(&command);
            let result = RegSetValueExW(key, value_name.as_ptr(), 0, REG_SZ, command_w.as_ptr() as *const u8, (command_w.len() * 2) as u32);
            RegCloseKey(key);
            result == ERROR_SUCCESS
        } else {
            if RegOpenKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, KEY_WRITE, &mut key) != ERROR_SUCCESS { return true; }
            let result = RegDeleteValueW(key, value_name.as_ptr());
            RegCloseKey(key);
            result == ERROR_SUCCESS || result == 2
        }
    }
}

unsafe fn get_module_filename(buffer: &mut [u16]) -> usize {
    extern "system" { fn GetModuleFileNameW(module: Hinstance, filename: *mut u16, size: u32) -> u32; }
    GetModuleFileNameW(null_mut(), buffer.as_mut_ptr(), buffer.len() as u32) as usize
}

fn startup_enabled() -> bool {
    let key_path = wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let value_name = wide("HealthReminder");
    let mut key = null_mut();
    unsafe {
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, KEY_READ, &mut key) != ERROR_SUCCESS { return false; }
        let mut kind = 0;
        let mut data_size = 0;
        let result = RegQueryValueExW(key, value_name.as_ptr(), null_mut(), &mut kind, null_mut(), &mut data_size);
        RegCloseKey(key);
        result == ERROR_SUCCESS
    }
}

fn notify(hwnd: Hwnd, title: &str, body: &str, icon_id: Option<usize>) {
    let balloon_icon = icon_id.map(|id| unsafe { LoadIconW(GetModuleHandleW(null()), id as *const u16) }).unwrap_or(null_mut());
    let info_flags = if balloon_icon.is_null() { NIIF_INFO } else { NIIF_USER };
    let mut data = NotifyIconDataW { cb_size: size_of::<NotifyIconDataW>() as u32, h_wnd: hwnd, u_id: 1, u_flags: NIF_INFO, u_callback_message: WM_APP_TRAY, h_icon: null_mut(), sz_tip: [0; 128], dw_state: 0, dw_state_mask: 0, sz_info: [0; 256], u_timeout_or_version: 5000, sz_info_title: [0; 64], dw_info_flags: info_flags, guid: [0; 16], h_balloon_icon: balloon_icon };
    copy_wide(title, &mut data.sz_info_title);
    copy_wide(body, &mut data.sz_info);
    unsafe { Shell_NotifyIconW(NIM_MODIFY, &mut data); }
}

fn app_version() -> String {
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap_or(0);
    format!("{}.{:02}", env!("CARGO_PKG_VERSION_MAJOR"), minor)
}

fn show_about(hwnd: Hwnd, locale: Locale) {
    let t = texts(locale);
    let caption = wide(t.about_title);
    let details = wide(&format!("{} {}\n{}", t.version_label, app_version(), t.about_attribution));
    unsafe { MessageBoxW(hwnd, details.as_ptr(), caption.as_ptr(), 0x40); }
}

fn copy_wide<const N: usize>(value: &str, destination: &mut [u16; N]) {
    for (index, code) in value.encode_utf16().take(N - 1).enumerate() { destination[index] = code; }
}

fn add_menu_item(menu: Hmenu, id: usize, label: &str) { let label_w = wide(label); unsafe { AppendMenuW(menu, MF_STRING, id, label_w.as_ptr()); } }

fn add_submenu(menu: Hmenu, submenu: Hmenu, label: &str) {
    let label_w = wide(label);
    unsafe { AppendMenuW(menu, MF_STRING | MF_POPUP, submenu as usize, label_w.as_ptr()); }
}

fn show_menu(hwnd: Hwnd, state: &AppState) {
    let t = texts(state.locale);
    unsafe {
        let menu = CreatePopupMenu();
        add_menu_item(menu, IDM_PAUSE, if state.paused { t.resume } else { t.pause });
        CheckMenuItem(menu, IDM_PAUSE, checked(state.paused));
        let settings_menu = CreatePopupMenu();
        add_menu_item(settings_menu, IDM_STARTUP, t.startup);
        CheckMenuItem(settings_menu, IDM_STARTUP, checked(state.startup));
        add_menu_item(settings_menu, IDM_TEST, t.test);
        let language_menu = CreatePopupMenu();
        add_menu_item(language_menu, IDM_LANG_EN, t.english);
        add_menu_item(language_menu, IDM_LANG_ES, t.spanish);
        add_menu_item(language_menu, IDM_LANG_PT, t.portuguese);
        CheckMenuItem(language_menu, match state.locale { Locale::English => IDM_LANG_EN, Locale::Spanish => IDM_LANG_ES, Locale::Portuguese => IDM_LANG_PT }, MF_CHECKED);
        add_submenu(settings_menu, language_menu, t.language);
        add_submenu(menu, settings_menu, t.settings);
        AppendMenuW(menu, MF_SEPARATOR, 0, null());
        let intervals_menu = CreatePopupMenu();
        let drink_menu = CreatePopupMenu();
        add_menu_item(drink_menu, IDM_DRINK_15, "15 min");
        add_menu_item(drink_menu, IDM_DRINK_30, "30 min");
        add_menu_item(drink_menu, IDM_DRINK_60, "60 min");
        CheckMenuItem(drink_menu, if state.drink_seconds == 900 { IDM_DRINK_15 } else if state.drink_seconds == 1800 { IDM_DRINK_30 } else { IDM_DRINK_60 }, MF_CHECKED);
        add_submenu(intervals_menu, drink_menu, t.drink);
        let exercise_menu = CreatePopupMenu();
        add_menu_item(exercise_menu, IDM_EXERCISE_30, "30 min");
        add_menu_item(exercise_menu, IDM_EXERCISE_60, "60 min");
        add_menu_item(exercise_menu, IDM_EXERCISE_120, "120 min");
        CheckMenuItem(exercise_menu, if state.exercise_seconds == 1800 { IDM_EXERCISE_30 } else if state.exercise_seconds == 3600 { IDM_EXERCISE_60 } else { IDM_EXERCISE_120 }, MF_CHECKED);
        add_submenu(intervals_menu, exercise_menu, t.exercise);
        add_submenu(menu, intervals_menu, t.intervals);
        AppendMenuW(menu, MF_SEPARATOR, 0, null());
        add_menu_item(menu, IDM_ABOUT, t.about);
        add_menu_item(menu, IDM_EXIT, t.exit);
        let mut point = Point { x: 0, y: 0 };
        GetCursorPos(&mut point);
        SetForegroundWindow(hwnd);
        TrackPopupMenu(menu, TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON, point.x, point.y, 0, hwnd, null());
        DestroyMenu(menu);
    }
}

fn tray_icon(hwnd: Hwnd, message: u32, icon: Hicon, tooltip: &str) {
    let mut data = NotifyIconDataW { cb_size: size_of::<NotifyIconDataW>() as u32, h_wnd: hwnd, u_id: 1, u_flags: NIF_MESSAGE | NIF_ICON | NIF_TIP, u_callback_message: WM_APP_TRAY, h_icon: icon, sz_tip: [0; 128], dw_state: 0, dw_state_mask: 0, sz_info: [0; 256], u_timeout_or_version: 0, sz_info_title: [0; 64], dw_info_flags: 0, guid: [0; 16], h_balloon_icon: null_mut() };
    copy_wide(tooltip, &mut data.sz_tip);
    unsafe { Shell_NotifyIconW(message, &mut data); }
}

unsafe fn health_icon(instance: Hinstance, paused: bool) -> Hicon {
    let icon_id = if paused { IDI_HEALTH_PAUSED } else { IDI_HEALTH };
    let icon = LoadIconW(instance, icon_id as *const u16);
    if icon.is_null() { LoadIconW(null_mut(), IDI_APPLICATION as *const u16) } else { icon }
}

unsafe fn update_tray_icon(hwnd: Hwnd, state: &AppState) {
    let icon = health_icon(GetModuleHandleW(null()), state.paused);
    let tooltip = tray_tooltip(state);
    tray_icon(hwnd, NIM_MODIFY, icon, &tooltip);
}

unsafe extern "system" fn window_proc(hwnd: Hwnd, message: u32, w_param: Wparam, l_param: Lparam) -> Lresult {
    let state_ptr = GetWindowLongPtrW(hwnd, -21) as *mut AppState;
    if message == WM_APP_TRAY {
        if l_param as usize == WM_RBUTTONUP && !state_ptr.is_null() { show_menu(hwnd, &*state_ptr); }
        return 0;
    }
    if message == WM_TIMER && w_param == TIMER_ID && !state_ptr.is_null() {
        let state = &mut *state_ptr;
        if !state.paused {
            let current = now();
            if current >= state.next_drink { let t = texts(state.locale); notify(hwnd, t.drink_title, t.drink_body, Some(IDI_WATER)); state.next_drink = current + Duration::from_secs(state.drink_seconds); }
            if current >= state.next_exercise { let t = texts(state.locale); notify(hwnd, t.exercise_title, t.exercise_body, Some(IDI_EXERCISE)); state.next_exercise = current + Duration::from_secs(state.exercise_seconds); }
        }
        update_tray_icon(hwnd, state);
        return 0;
    }
    if message == WM_COMMAND && !state_ptr.is_null() {
        let state = &mut *state_ptr;
        match w_param & 0xffff {
            IDM_PAUSE => state.paused = !state.paused,
            IDM_TEST => { let t = texts(state.locale); notify(hwnd, t.test_title, t.test_body, None); }
            IDM_STARTUP => { let enabled = !state.startup; if set_startup(enabled) { state.startup = enabled; } else { let t = texts(state.locale); let caption = wide(t.name); let body = wide(t.error); MessageBoxW(hwnd, body.as_ptr(), caption.as_ptr(), 0x10); } }
            IDM_DRINK_15 => { state.drink_seconds = 900; state.next_drink = now() + Duration::from_secs(900); }
            IDM_DRINK_30 => { state.drink_seconds = 1800; state.next_drink = now() + Duration::from_secs(1800); }
            IDM_DRINK_60 => { state.drink_seconds = 3600; state.next_drink = now() + Duration::from_secs(3600); }
            IDM_EXERCISE_30 => { state.exercise_seconds = 1800; state.next_exercise = now() + Duration::from_secs(1800); }
            IDM_EXERCISE_60 => { state.exercise_seconds = 3600; state.next_exercise = now() + Duration::from_secs(3600); }
            IDM_EXERCISE_120 => { state.exercise_seconds = 7200; state.next_exercise = now() + Duration::from_secs(7200); }
            IDM_LANG_EN => state.locale = Locale::English,
            IDM_LANG_ES => state.locale = Locale::Spanish,
            IDM_LANG_PT => state.locale = Locale::Portuguese,
            IDM_ABOUT => show_about(hwnd, state.locale),
            IDM_EXIT => { DestroyWindow(hwnd); }
            _ => {}
        }
        if (w_param & 0xffff) != IDM_EXIT { save_settings(state); update_tray_icon(hwnd, state); }
        return 0;
    }
    if message == WM_DESTROY {
        KillTimer(hwnd, TIMER_ID);
        tray_icon(hwnd, NIM_DELETE, null_mut(), "");
        if !state_ptr.is_null() { drop(Box::from_raw(state_ptr)); SetWindowLongPtrW(hwnd, -21, 0); }
        PostQuitMessage(0);
        return 0;
    }
    DefWindowProcW(hwnd, message, w_param, l_param)
}

fn main() {
    let locale = load_locale();
    let t = texts(locale);
    let class_name = wide("HealthReminderWindow");
    unsafe {
        if SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) == 0 { SetProcessDPIAware(); }
        let instance = GetModuleHandleW(null());
        let icon = health_icon(instance, false);
        let class = WndClassW { style: 0, lpfn_wnd_proc: Some(window_proc), cb_cls_extra: 0, cb_wnd_extra: 0, h_instance: instance, h_icon: icon, h_cursor: null_mut(), hbr_background: null_mut(), lpsz_menu_name: null(), lpsz_class_name: class_name.as_ptr() };
        if RegisterClassW(&class) == 0 { return; }
        let hwnd = CreateWindowExW(0, class_name.as_ptr(), wide(t.name).as_ptr(), 0, 0, 0, 0, 0, null_mut(), null_mut(), instance, null_mut());
        if hwnd.is_null() { return; }
        let drink_seconds = valid_drink_interval(read_setting("DrinkIntervalSeconds", 1800));
        let exercise_seconds = valid_exercise_interval(read_setting("ExerciseIntervalSeconds", 3600));
        let paused = read_setting("Paused", 0) != 0;
        let startup = startup_enabled();
        let state = Box::new(AppState { locale, drink_seconds, exercise_seconds, next_drink: now() + Duration::from_secs(drink_seconds), next_exercise: now() + Duration::from_secs(exercise_seconds), paused, startup });
        save_settings(&state);
        let tray_icon_handle = health_icon(instance, state.paused);
        let tooltip = tray_tooltip(&state);
        SetWindowLongPtrW(hwnd, -21, Box::into_raw(state) as isize);
        tray_icon(hwnd, NIM_ADD, tray_icon_handle, &tooltip);
        SetTimer(hwnd, TIMER_ID, 15_000, null_mut());
        let mut message = Message { hwnd: null_mut(), message: 0, w_param: 0, l_param: 0, time: 0, point: Point { x: 0, y: 0 } };
        while GetMessageW(&mut message, null_mut(), 0, 0) > 0 { DispatchMessageW(&message); }
    }
}
