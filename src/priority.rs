#[cfg(windows)]
pub fn set_low_priority() {
    use windows::Win32::System::Threading::{
        BELOW_NORMAL_PRIORITY_CLASS, GetCurrentProcess, SetPriorityClass,
    };
    unsafe {
        let handle = GetCurrentProcess();
        let _ = SetPriorityClass(handle, BELOW_NORMAL_PRIORITY_CLASS);
    }
}

#[cfg(not(windows))]
pub fn set_low_priority() {
    // Non‑Windows platforms: no operation
}
