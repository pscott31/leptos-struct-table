# Changelog

## [0.6.0] - 2023-11-02

### New Feature 🎉

- Support for generic structs

### Fix 🐛

- Fixed `#[table(skip_sort)]` on fields

## [0.5.0] - 2023-10-20

### Breaking Changes 🛠️

- Added `on_change` events to support editable data (see new editable example)

### Fixes 🐛

- Fixed selection with `key`s that are not `Copy`

### Other Changes

- Modified REST example to include sorting


## [0.4.0] - 2023-10-02

- Updated to leptos 0.5

## [0.3.0]

- Updated to leptos 0.4

## [0.2.0]

- Updated to leptos 0.3
- Deactivated `default-features` of leptos
- New class provider `BootstrapClassesPreset`
- New example `bootstrap`
- Added `thead` and `tbody` with customizable renderers
- Added `getter` and `FieldGetter<T>` with new example