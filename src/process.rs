use std::{ffi::OsStr, os::windows::prelude::OsStrExt, ptr::null_mut};
use anyhow::{Result, bail};
use winapi::{um::{winbase::STARTUPINFOEXW, processthreadsapi::{InitializeProcThreadAttributeList, LPPROC_THREAD_ATTRIBUTE_LIST, PROCESS_INFORMATION, CreateProcessW}, errhandlingapi::GetLastError}, shared::{minwindef::BOOL, ntdef::LPCWSTR}};

pub fn create_subprocess(exec: &String) -> Result<()> {
    let lpFile = OsStr::new(&exec).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    let mut si: STARTUPINFOEXW = unsafe { std::mem::zeroed() };
    si.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXW>() as u32;
    let mut lp_size: usize = 0;
    let mut success: BOOL;
    success = unsafe {InitializeProcThreadAttributeList(
        0 as LPPROC_THREAD_ATTRIBUTE_LIST,
        1,
        0,
        &mut lp_size,
    )};
    if success == 1 || lp_size == 0 {
        let err = unsafe {GetLastError()};
        bail!(
            "Can't calculate the number of bytes for the attribute list, {}",
            err
        );
    }

    let mut lp_attribute_list: Box<[u8]> = vec![0; lp_size].into_boxed_slice();
    si.lpAttributeList = LPPROC_THREAD_ATTRIBUTE_LIST::from(lp_attribute_list.as_mut_ptr().cast::<_>());

    success = unsafe { InitializeProcThreadAttributeList(si.lpAttributeList, 1, 0, &mut lp_size) };
    if success == 0 {
        let err = unsafe {GetLastError()};
        bail!("Can't setup attribute list, {}", err);
    }

    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    success = unsafe { CreateProcessW(lpFile.as_ptr() as LPCWSTR, null_mut(), null_mut(), null_mut(), 1, 0, null_mut(), null_mut(), &mut si.StartupInfo, &mut pi) };
    if success == 0 {
       let error = unsafe {GetLastError()};
       bail!("Cannot create process: {}", error);
    }
    Ok(())
}