# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/); versions follow [semver](https://semver.org).

## [0.2.0] - 2026-11-04

### Changed
- **Breaking:** `utoipa` 4 → 5, requested by the ecom-engine dogfood
  cycle (the engine is on utoipa 5, semver-incompatible with the
  openapi feature's ^4 pin). `ToSchema` derives migrated to the utoipa
  5 derive surface.

## [0.1.0] - 2026-09-05

### Added
- Initial public release.
