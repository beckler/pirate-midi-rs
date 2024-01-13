# Changelog

## [1.4.0](https://github.com/beckler/pirate-midi-rs/compare/v1.3.1...v1.4.0) (2024-01-13)


### Features

* added additional support for bridge 2.0 ([ff76c52](https://github.com/beckler/pirate-midi-rs/commit/ff76c5299bb8181e6ab225067e2475ba9cf06801))


### Bug Fixes

* added additional changes to support bridge OS 2.0 ([302c542](https://github.com/beckler/pirate-midi-rs/commit/302c5423c4811a1b5cc7c90ecb98e4ff81851515))
* migrating to upstream serial port fix ([ef58525](https://github.com/beckler/pirate-midi-rs/commit/ef5852553a0c1c748e1d02dcc35e34201de1e5b8))

## [1.3.1](https://github.com/beckler/pirate-midi-rs/compare/v1.3.0...v1.3.1) (2023-12-11)


### Bug Fixes

* clippy warnings ([ef8f1a8](https://github.com/beckler/pirate-midi-rs/commit/ef8f1a8d0cda58f6689ccb40d003ddc84db8c37b))

## [1.3.0](https://github.com/beckler/pirate-midi-rs/compare/v1.2.1...v1.3.0) (2023-12-11)


### Features

* added support for bridge os 2.0 commands ([37ce8aa](https://github.com/beckler/pirate-midi-rs/commit/37ce8aa786a1a2d3af98204850807253e51c4c77))


### Bug Fixes

* added uniqueDeviceId as an alias for uID ([8338313](https://github.com/beckler/pirate-midi-rs/commit/8338313689d8ff81bb1c69b158602f0483543b07))

## [1.2.1](https://github.com/beckler/pirate-midi-rs/compare/v1.2.0...v1.2.1) (2023-03-04)


### Bug Fixes

* switching to a version of serialport that has a small custom change ([08ebe67](https://github.com/beckler/pirate-midi-rs/commit/08ebe6729be1b586d14851fec47a55e486ac9e69))

## [1.2.0](https://github.com/beckler/pirate-midi-rs/compare/v1.1.4...v1.2.0) (2023-02-17)


### Features

* will now accept a serialport builder instead of just creating it's own ([8455242](https://github.com/beckler/pirate-midi-rs/commit/8455242bc5cd70460e9266462dbdc844231752b7))

## [1.1.4](https://github.com/beckler/pirate-midi-rs/compare/v1.1.3...v1.1.4) (2022-10-11)


### Bug Fixes

* added serialize trait to check resposne ([d3203c2](https://github.com/beckler/pirate-midi-rs/commit/d3203c269c955b4f4bb3ac0b2290835996b413fa))

## [1.1.3](https://github.com/beckler/pirate-midi-rs/compare/v1.1.2...v1.1.3) (2022-08-19)


### Bug Fixes

* added explicit DTR flag for windows ([0a4010b](https://github.com/beckler/pirate-midi-rs/commit/0a4010b7fe42d69135d9d97458b119e04d9ad37d))
* fixed fmt issues ([134e7b8](https://github.com/beckler/pirate-midi-rs/commit/134e7b84cd22fbe23f809cc85c2879822d4f202a))

## [1.1.2](https://github.com/beckler/pirate-midi-rs/compare/v1.1.1...v1.1.2) (2022-08-19)


### Bug Fixes

* added more tracing ([5197335](https://github.com/beckler/pirate-midi-rs/commit/5197335dbdc6114af8caff27e9db1823d0d59c26))

## [1.1.1](https://github.com/beckler/pirate-midi-rs/compare/v1.1.0...v1.1.1) (2022-08-18)


### Bug Fixes

* fixed clippy warnings ([af83227](https://github.com/beckler/pirate-midi-rs/commit/af832276b1550532d3bbe0c2a8ceb3b5f0630c93))

## [1.1.0](https://github.com/beckler/pirate-midi-rs/compare/v1.0.0...v1.1.0) (2022-08-18)


### Features

* update readme ([1478464](https://github.com/beckler/pirate-midi-rs/commit/1478464e4c3f0727a43b6cd6fb45fb7750501846))


### Bug Fixes

* added tracing ([fa3087f](https://github.com/beckler/pirate-midi-rs/commit/fa3087f9e3097eae412ca7ddfdaf878887b03a09))

## [1.0.0](https://github.com/beckler/pirate-midi-rs/compare/v0.1.1...v1.0.0) (2022-08-11)


### ⚠ BREAKING CHANGES

* cleaned up docs, moved models to messages

### Bug Fixes

* cleaned up docs, moved models to messages ([f0a5dd1](https://github.com/beckler/pirate-midi-rs/commit/f0a5dd17a47400b47779cf253f82b28fda899b5e))

## [0.1.1](https://github.com/beckler/pirate-midi-rs/compare/v0.1.0...v0.1.1) (2022-08-11)


### Bug Fixes

* forcing release ([350d580](https://github.com/beckler/pirate-midi-rs/commit/350d580164bbb43449e099dcdea3a37404e889c7))

## 0.1.0 (2022-08-11)


### Features

* initial commit ([307da4b](https://github.com/beckler/pirate-midi-rs/commit/307da4b198f8a48cfc72fca17ebbc8136f072abc))


### Bug Fixes

* identified clippy issues ([b3cadfc](https://github.com/beckler/pirate-midi-rs/commit/b3cadfc0b0a90ae7e93443e0bc4f4689f7dedb11))
* misnamed variables on bank settings ([d4465e6](https://github.com/beckler/pirate-midi-rs/commit/d4465e6d9c3aa55d82d440b7e41b77331676043b))
