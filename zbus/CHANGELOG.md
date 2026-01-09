# zbus Changelog

## 5.13.0 - 2026-01-09

### Added
- ✨ Add crate attribute for custom crate paths.


### Changed
- 🎨 Format all files (rust 1.85).


### Fixed
- 🚑️ Send on unix sockets w/ `MSG_NOSIGNAL` flag enabled. #1657
- 🐛 Fix `get_machine_id` for macOS.


### Other
- 🧱 Fix all clippy warnings (rust 1.85).
- 🧑‍💻 Bump rust version to 1.85.
- 🔊 lower trace/instrument verbosity.


### Testing
- ✅ Add introspection test for out_args with single output.
- ✅ Remove unused imports from tests.

