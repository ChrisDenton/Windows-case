use std::collections::BTreeMap;

static CASE_FOLDING: &str = include_str!("../ucd/5.1.0/CaseFolding.txt");

// Manual adjustments needed to match Windows.
const ADJUSTMENTS: &[(u16, u16)] = &[
	(0x6b, 0x4b),
	(0x73, 0x53),
	(0xdf, 0xdf),
	(0xe5, 0xc5),
	(0x1c6, 0x1c4),
	(0x1c9, 0x1c7),
	(0x1cc, 0x1ca),
	(0x1f3, 0x1f1),
	(0x3b2, 0x392),
	(0x3b5, 0x395),
	(0x3b8, 0x398),
	(0x3b9, 0x399),
	(0x3ba, 0x39a),
	(0x3c0, 0x3a0),
	(0x3c1, 0x3a1),
	(0x3c3, 0x3a3),
	(0x3c6, 0x3a6),
	(0x3c9, 0x3a9),
	(0x1e61, 0x1e60),
];

type Map = BTreeMap<u16, u16>;

/// Generate mappings from Unicode 5.1.0 CaseFolding data.
/// This applies only to UTF-16 code units and has aditional adjustments
/// on top of the Unicode data. If the UTF-16 code unit does not appear in this
/// mapping then it maps to itself.
pub fn gen_mappings() -> Map {
	try_gen_mappings(CASE_FOLDING, ADJUSTMENTS).expect("Failed to parse CaseFolding.txt data")
}

// TODO: Return a proper error type.
pub fn try_gen_mappings(data: &str, adjustment: &[(u16, u16)]) -> Option<Map> {
	let mut map = BTreeMap::new();
	for line in data.lines() {
		if line.is_empty() || line.starts_with("#") {
			continue;
		}
		// Get the three columns in the table (ignoring the trailing comment).
		let mut columns = line.split("; ");
		let (code, status, mapping) = (columns.next()?, columns.next()?, columns.next()?);

		// Simple casefolding and only code points <= U+FFFF.
		if (status == "C" || status == "S") && code.len() == 4 && mapping.len() == 4 {
			// NOTE: Windows uses backwards conversions for reasons lost to history.
			map.insert(from_hex(mapping)?, from_hex(code)?);
		}
	}
	// Apply the adjusts on top of the unicode data.
	for (a, b) in adjustment.iter().copied() {
		map.insert(a, b);
	}
	Some(map)
}
fn from_hex(hex: &str) -> Option<u16> {
	u16::from_str_radix(hex, 16).ok()
}
