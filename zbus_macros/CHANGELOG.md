# zbus_macros Changelog

## 5.13.0 - 2026-01-09

### Added
- ✨ add special handling for ao DBus signatures. #332
- ✨ Add crate attribute for custom crate paths.


### Changed
- 🎨 Format all files (rust 1.85).
- ♻️ Replace panic with proper Error in introspect_add_output_args.
- ♻️ rename parameters / variables.


### Fixed
- 🐛 zbus_macros shouldn't set features on zbus.
- 🐛 Apply out_args to single outputs in introspection XML. #1599
- 🐛 ignore r# prefix in parameter names. #158
- 🐛 ignore r# prefix in method names. #214


### Other
- 🧱 Fix all clippy warnings (rust 1.85).
- 🧑‍💻 Bump rust version to 1.85.

