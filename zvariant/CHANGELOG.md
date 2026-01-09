# zvariant Changelog

## 5.9.0 - 2026-01-09

### Added
- ✨ Implement `TryFrom<&Value>` for tuples.
- ✨ Add signature! macro for compile-time validation. #984


### Changed
- 🎨 Format all files (rust 1.85).
- ♻️ Use signature! macro in tests.


### Dependencies
- ⬆️ Update endi to v1.1.1 (#1583).


### Fixed
- 🐛 Don't impl Type for dicts with non-basic keys. #1637


### Other
- 🧱 Fix all clippy warnings (rust 1.85).
- 🧑‍💻 Bump rust version to 1.85.
- 🚸 Implement `to_string_lossy` for `FilePath`.


### Testing
- ✅ Remove unused imports from tests.

