# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.0.12] - 2024-10-28
### Added
- Added support for keepalive messages so that connections are kept open

## [0.0.11] - 2024-10-28
### Added
- Fixed bug in handling of client proxy IP - need to use IP not port ;-)

## [0.0.10] - 2024-10-25
### Added
- Fixed bug in parsing header proxy IP address

## [0.0.9] - 2024-10-25
### Added
- Fixed bug in parsing command line options for bridge
- Added support for getting the client IP address from a proxy header (e.g. `X-Forwarded-For`)
- Cleaned up port handling, so URLs with default ports don't have the ports specified

## [0.0.8] - 2024-10-24
### Added
- Added names for the ports in the helm charts

## [0.0.7] - 2024-10-24
### Added
- Added a healthcheck server to simplify pod healthchecks
- Updated helm charts to use the healthcheck server, plus expose the bridge server port

## [0.0.6] - 2024-10-23
### Added
- Separated out build artefacts so that they can be picked up by the rest of the build

## [0.0.5] - 2024-10-23
### Added
- Fixing generation and attestation of SBOMs for container images (finally!)

## [0.0.4] - 2024-10-23
### Added
- Fixing release issues, and beginning work on the workflow for the Python module

## [0.0.3] - 2024-10-23
### Added
- Fixing the attestations so that SBOMs are correctly generated for container images.

## [0.0.2] - 2024-10-23
### Added
- Fixing the helm charts so that they version numbers are correctly set.

## [0.0.1] - 2024-10-23
### Changed
- Initial release
  This is an initial alpha release of the OpenPortal project. It is not yet feature complete and is not recommended for production use.

[0.0.12]: https://github.com/isambard-sc/openportal/releases/tag/0.0.12
[0.0.11]: https://github.com/isambard-sc/openportal/releases/tag/0.0.11
[0.0.10]: https://github.com/isambard-sc/openportal/releases/tag/0.0.10
[0.0.9]: https://github.com/isambard-sc/openportal/releases/tag/0.0.9
[0.0.8]: https://github.com/isambard-sc/openportal/releases/tag/0.0.8
[0.0.7]: https://github.com/isambard-sc/openportal/releases/tag/0.0.7
[0.0.6]: https://github.com/isambard-sc/openportal/releases/tag/0.0.6
[0.0.5]: https://github.com/isambard-sc/openportal/releases/tag/0.0.5
[0.0.4]: https://github.com/isambard-sc/openportal/releases/tag/0.0.4
[0.0.3]: https://github.com/isambard-sc/openportal/releases/tag/0.0.3
[0.0.2]: https://github.com/isambard-sc/openportal/releases/tag/0.0.2
[0.0.1]: https://github.com/isambard-sc/openportal/releases/tag/0.0.1
