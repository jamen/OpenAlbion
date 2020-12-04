use std::path::{Path, PathBuf};

use winapi::shared::minwindef::*;
use winapi::um::handleapi::*;
use winapi::um::libloaderapi::*;
use winapi::um::memoryapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::synchapi::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use std::ffi::CString;
use std::mem;
use std::ptr::null_mut;

// Checks for a file from the arguments, working directory, or executable's directory.
fn find_file(
    file_name: &str,
    arg: Option<&String>,
    current_dir: &Path,
    exe_dir: &Path,
) -> Option<PathBuf> {
    if let Some(arg) = arg {
        let arg_as_path = PathBuf::from(arg);

        if arg_as_path.exists() {
            return Some(arg_as_path);
        }
    }

    let current_dir_path = current_dir.join(file_name);

    if current_dir_path.exists() {
        return Some(current_dir_path);
    }

    let exe_dir_path = exe_dir.join(file_name);

    if exe_dir_path != current_dir_path && exe_dir_path.exists() {
        return Some(exe_dir_path);
    }

    None
}

fn main() {
    unsafe {
        let args: Vec<String> = std::env::args().skip(1).collect();

        let current_dir = std::env::current_dir().unwrap();
        let exe_path = std::env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        let fable_exe_path = find_file("Fable.exe", args.get(0), current_dir.as_path(), exe_dir)
            .expect("Couldn't find Fable.exe");

        let tlse_dll_path = find_file("tlse.dll", args.get(1), current_dir.as_path(), exe_dir)
            .expect("Couldn't find tlse.dll");

        let fable_exe_path_str = fable_exe_path.to_str().unwrap();
        let fable_exe_path_cstr = CString::new(fable_exe_path_str).unwrap();

        let tlse_dll_path_str = tlse_dll_path.to_str().unwrap();
        let tlse_dll_path_cstr = CString::new(tlse_dll_path_str).unwrap();
        let tlse_dll_path_bytes = tlse_dll_path_cstr.to_bytes_with_nul();

        let mut process_info: PROCESS_INFORMATION = Default::default();
        let mut startup_info: STARTUPINFOA = Default::default();

        startup_info.cb = mem::size_of::<STARTUPINFOA>() as u32;

        if CreateProcessA(
            null_mut(),
            fable_exe_path_cstr.as_ptr() as LPSTR,
            null_mut(),
            null_mut(),
            0,
            CREATE_SUSPENDED,
            null_mut(),
            null_mut(),
            &mut startup_info,
            &mut process_info,
        ) == 0
        {
            panic!("Failed to create Fable.exe process.")
        }

        if !process_info.hProcess.is_null() {
            CloseHandle(process_info.hProcess);
        }

        if !process_info.hThread.is_null() {
            CloseHandle(process_info.hThread);
        }

        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, process_info.dwProcessId);
        let thread_handle = OpenThread(THREAD_ALL_ACCESS, 0, process_info.dwThreadId);

        let tlse_dll_path_remote = VirtualAllocEx(
            process_handle,
            null_mut(),
            tlse_dll_path_bytes.len(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        );

        if tlse_dll_path_remote.is_null() {
            panic!("DLL injection failed: Remote memory allocation error.");
        }

        if WriteProcessMemory(
            process_handle,
            tlse_dll_path_remote,
            tlse_dll_path_bytes.as_ptr() as LPVOID,
            tlse_dll_path_bytes.len(),
            null_mut(),
        ) == 0
        {
            panic!("DLL injection failed: Failed to write DLL path in process.");
        }

        let module_name = CString::new("kernel32.dll").unwrap();
        let proc_name = CString::new("LoadLibraryA").unwrap();

        let module_handle = GetModuleHandleA(module_name.as_ptr());

        let remote_proc = GetProcAddress(module_handle, proc_name.as_ptr());
        let remote_proc =
            *(&remote_proc as *const _ as *const unsafe extern "system" fn(LPVOID) -> DWORD);

        let remote_thread: HANDLE = CreateRemoteThread(
            process_handle,
            null_mut(),
            0,
            Some(remote_proc),
            tlse_dll_path_remote,
            0,
            null_mut(),
        );

        WaitForSingleObject(remote_thread, INFINITE);

        VirtualFreeEx(process_handle, tlse_dll_path_remote, 0, MEM_RELEASE);

        // TODO: Use the remote LoadLibraryA return value?
        // NOTE: Stackoverflow commenter warned "GetExitCodeThread(t1, &retVal) and returned 4294967295 (retVal being a DWORD). The actual return value in the thread was -1. I just figured out the ints rebounded to negatives. Sigh... – Sefu Aug 17 '11 at 23:05"
        let mut ret_val = 0;

        if GetExitCodeThread(remote_thread, &mut ret_val) == 0 {
            panic!("The DLL injection failed: Thread's exit code unavailable.");
        }

        if ResumeThread(thread_handle) == 4294967295 {
            println!("Warning: Failed to resume process.");
        }

        CloseHandle(process_info.hThread);
        CloseHandle(process_info.hProcess);
        CloseHandle(thread_handle);
        CloseHandle(process_handle);
    }
}
