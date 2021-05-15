# Windows Casefolding

Windows uses a case-insensitive algorithm to compare some kinds of strings (e.g. environment variables and paths).
This algorithm is "interesting". It iterates by UTF-16 code units (not code points) comparing the binary values of each. Where it gets interesting is that it first "upper" cases the code units but in a weird way. It does so by doing the reverse of normal Unicode case-folding. This does not make much sense to me but I'd assume it's for historic reasons.

On my system it appears to use Unicode 5.1.0 reverse case-folding but with some fixups applied, perhaps because misapplying Unicode case-folding can produce some weird results. This library parses the Unicode case-folding rules and applies the necessary fixups.

However, it's not safe to assume that this will work correctly on newer or older systems than my own. Windows versions can (and have) modified the uppercase mappings. It's also not safe to assume it will work the same in all contexts even on the same system. For example, when an NTFS volume is first formatted, it will have uppercase mappings written to a special file. This file remains the same throughout the life of the volume. So even different volumes on the same machine can have slightly different casing rules.
