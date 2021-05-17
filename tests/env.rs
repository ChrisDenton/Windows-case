#![cfg(windows)]

use std::ptr::null;

const VALUE: u16 = b'1' as _;

#[test]
#[ignore = "Exhaustively testing environment variable mappings may be slow"]
fn environment_case() -> Result<(), ()> {
	let casefold = wincase::Casefold::new();

	// Make sure we're starting clean.
	for key in 1..=u16::MAX {
		unset_env(key)?;
	}
	// Test every UTF-16 code unit, ignoring surrogates.
	for key in 1..=u16::MAX {
		if key >= 0xD800 && key <= 0xDFFF {
			continue;
		}
		set_env(key, VALUE)?;
		for case in 1..=u16::MAX {
			if case >= 0xD800 && case <= 0xDFFF {
				continue;
			}
			if get_env(case) == Ok(VALUE) {
				assert!(casefold.equal(key, case));
			}
		}
		unset_env(key)?;
	}
	Ok(())
}

// Very primitive wrappers around OS functions.
// I'm using the Windows API directly (instead of stdlib) because I want to
// avoid even the possibility of the std interfering with this test (however unlikly).

fn get_env(key: u16) -> Result<u16, ()> {
	let key = [key, 0];
	unsafe { get_environment_variable(&key) }
}

fn set_env(key: u16, value: u16) -> Result<(), ()> {
	let key = [key, 0];
	let value = [value, 0];

	unsafe { set_environment_variable(&key, Some(&value)) }
}
fn unset_env(key: u16) -> Result<(), ()> {
	let key = [key, 0];
	unsafe { set_environment_variable(&key, None) }
}

// Strings must be null terminated.
unsafe fn set_environment_variable(key: &[u16], value: Option<&[u16]>) -> Result<(), ()> {
	let value = value.map(|v| v.as_ptr()).unwrap_or(null());
	let result = SetEnvironmentVariableW(key.as_ptr(), value);
	if result == 0 {
		Err(())
	} else {
		Ok(())
	}
}
// Strings must be null terminated.
unsafe fn get_environment_variable(key: &[u16]) -> Result<u16, ()> {
	let mut buff = [0_u16; 2];
	let result = GetEnvironmentVariableW(key.as_ptr(), buff.as_mut_ptr(), buff.len() as u32);
	if result != 0 && result as usize <= buff.len() {
		Ok(buff[0])
	} else {
		Err(())
	}
}

#[link(name = "kernel32")]
extern "system" {
	fn SetEnvironmentVariableW(lpName: *const u16, lpValue: *const u16) -> i32;
	fn GetEnvironmentVariableW(lpName: *const u16, lpBuffer: *mut u16, nSize: u32) -> u32;
}
