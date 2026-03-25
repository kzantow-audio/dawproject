# Changelog

## [0.10.0](https://github.com/hifa-lang/dawproject/compare/0.9.0...0.10.0)

### Breaking Changes

- Replaced `yaserde` (hifa_yaserde) with `quick-xml` + `serde` for XML serialization/deserialization.
- Replaced `hifa-xml-schema-derive` with `xsd-parser` for XSD-to-Rust code generation.
- Generated structs are now **flat** — inherited XSD fields are inlined directly (no more `.base.base` nesting).
- Field paths changed: e.g. `metadata.content.title` → `metadata.title`, `project.content.version` → `project.version`.
- Trait APIs simplified: `NameableTrait::get_name()` now returns directly without intermediate `get_nameable()`.

### Added

- Build-time XSD→Rust code generation via `xsd-parser` with `serde` + `quick-xml` backend.
- Generated code lives in `src/generated/` for IDE browsability.
- `skip_serializing_if` on `Option`/`Vec` fields for clean round-trip XML serialization.
- `Clone`, `PartialEq` derives on all generated types.

### Changed

- Updated `zip` dependency from 2.6.1 to 8.4.
- Updated `quick-xml` to 0.38 (aligned with xsd-parser).
- Updated `thiserror` to 2.0.18.

### Removed

- Removed `hifa_yaserde`, `hifa_yaserde_derive`, `hifa-xml-schema`, `hifa-xml-schema-derive`, `xml-rs` dependencies.

## [0.9.0](https://github.com/hifa-lang/dawproject/compare/0.8.2...0.9.0) - 2025-04-24

- Update dependencies.
