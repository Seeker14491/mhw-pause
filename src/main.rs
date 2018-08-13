#![cfg(windows)]

#[macro_use]
extern crate failure;
extern crate winapi;

use failure::{Error, ResultExt};
use std::{ffi::OsStr, io, mem, os::windows::ffi::OsStrExt};
use winapi::{
    shared::{
        minwindef::{FALSE, MAX_PATH, TRUE},
        ntdef::NTSTATUS,
    },
    um::{
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        tlhelp32::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
    },
};

#[link(name = "ntdll")]
extern "system" {
    fn NtSuspendProcess(process_handle: HANDLE) -> NTSTATUS;
    fn NtResumeProcess(process_handle: HANDLE) -> NTSTATUS;
}

const TARGET_EXE: &str = "MonsterHunterWorld.exe";

fn main() {
    if let Err(err) = run() {
        println!("Error: {}\nPress enter to exit...", err);
        pause_newline();
    }
}

fn run() -> Result<(), Error> {
    let f = |handle| unsafe {
        NtSuspendProcess(handle);

        println!("Suspended {}; press Enter to resume...", TARGET_EXE);
        pause_newline();

        NtResumeProcess(handle);
    };

    unsafe {
        call_on_process(TARGET_EXE, f)
            .with_context(|_| format_err!("couldn't find running {}", TARGET_EXE))?;
    }

    Ok(())
}

fn pause_newline() {
    io::stdin().read_line(&mut String::new()).ok();
}

unsafe fn call_on_process<T, F>(target_exe: T, f: F) -> Result<(), Error>
where
    T: AsRef<OsStr>,
    F: FnOnce(HANDLE),
{
    let target_exe = target_exe.as_ref();
    let mut entry = PROCESSENTRY32W {
        dwSize: mem::size_of::<PROCESSENTRY32W>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; MAX_PATH],
    };

    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if Process32FirstW(snapshot, &mut entry) == TRUE {
        while Process32NextW(snapshot, &mut entry) == TRUE {
            if entry
                .szExeFile
                .iter()
                .zip(target_exe.encode_wide())
                .all(|(&a, b)| a == b)
            {
                let h_process = OpenProcess(PROCESS_ALL_ACCESS, FALSE, entry.th32ProcessID);

                f(h_process);

                CloseHandle(h_process);
                CloseHandle(snapshot);
                return Ok(());
            }
        }
    }

    Err(failure::err_msg("target exe not found"))
}
