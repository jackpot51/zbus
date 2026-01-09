# zbus_names Changelog

## 4.3.0 - 2026-01-09

### Added
- ✨ Implement Borrow for Owned* types.


### Changed
- ♻️ Reduce code duplication with `define_name_type_impls!` macro.
- 🎨 Format all files (rust 1.85).
- 🚚 Update name of Github space from dbus2 to z-galaxy.


### Documentation
- 📝 doc typo, Error names have same constraints as *interface* names.


### Fixed
- 🩹 Don't use workspace for local deps.


### Other
- 👽️ Use `std::hint::black_box` in benchmarks code.
- 🧑‍💻 Use workspace dependencies.


### Removed
- ➖ Drop `static_assertions` dep.

