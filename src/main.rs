use wincase;
use std::{collections::BTreeMap, io::{self, Read, Write}, path::Path, fs};

struct ClosestMatch {
    version: String,
    map: BTreeMap<u16,u16>
}

/// Learn how to generate the uppercase mappings from unicode tables.
fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut closest: Option<ClosestMatch> = None;
    write_str(&mut out, "Generating OS mapping... ")?;
    
    let os_mapping = gen_os_map();

    writeln!(&mut out, "Done.")?;
    write_str(&mut out, "Finding closest match... ")?;
    
    for entry in fs::read_dir(Path::new("ucd"))? {
        let entry = entry?;
        let ucd_version = entry.file_name().to_string_lossy().into_owned();

        let mut p = entry.path();
        p.push("CaseFolding.txt");
        
        let mut f = fs::File::open(&p)?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;

        let ucd_mapping = wincase::try_gen_mappings(&data, &[]).unwrap();
        let mut diff = BTreeMap::new();
        for lower in 0..=u16::MAX {
            // Skip surrogates.
            if lower >= 0xD800 && lower <= 0xDFFF {
                continue;
            }
            let a = os_mapping.get(&lower).unwrap_or(&lower);
            let b = ucd_mapping.get(&lower).unwrap_or(&lower);
            if a != b {
                diff.insert(lower, *a);
            }
        }
        let current = ClosestMatch {
            version: ucd_version,
            map: diff,
        };
        if let Some(prev) = closest.as_ref() {
            if current.map.len() < prev.map.len() {
                closest = Some(current);
            }
        } else {
            closest = Some(current)
        }
    }
    let closest = closest.unwrap();
    writeln!(&mut out, "Done.\n")?;
    writeln!(&mut out, "Unicode version {}", closest.version)?;
    writeln!(&mut out, "Fixups:")?;
    for (k, v) in &closest.map {
        writeln!(&mut out, "\t({:#06x}, {:#06x}),", k, v)?;
    }
    writeln!(&mut out, "Total fixups: {}", closest.map.len())?;
    Ok(())
}


/// Get the OS' mapping.
pub fn gen_os_map() -> BTreeMap<u16,u16> {
	let mut map = BTreeMap::new();
	for lower in 0..=u16::MAX {
		// Skip surrogates.
		if lower >= 0xD800 && lower <= 0xDFFF {
			continue;
		}
		let upper = upcase_wchar(lower);
		if upper != lower {
			map.insert(lower, upper);
		}
	}
	map
}

// Both writes and flushes the output.
fn write_str<T: Write>(mut f: T, s: &str) -> io::Result<()> {
    f.write_all(s.as_bytes())?;
    f.flush()?;
    Ok(())
}

fn upcase_wchar(c: u16) -> u16 {
	unsafe { RtlUpcaseUnicodeChar(c) }
}

#[link(name = "ntdll")]
extern "system" {
	fn RtlUpcaseUnicodeChar(SourceCharacter: u16) -> u16;
}
