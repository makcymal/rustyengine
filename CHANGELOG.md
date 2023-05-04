# Changelog

All notable changes to this project will be documented in this file.

## [0.0.3] - 2023-05-04

### Added
- `Ray` struct using `Point` and `Vector` in the given `CoordSys`
- `IdSet` uses crate `uuid`. More specifically it uses UUID standard v4. It's prohibited to mutate `Id`. 
- `EntityCore` used in every particular `Entity` (following principle Composition Over Inheritance). It's intended to create it within `Game` instance
- Properties in `EntityCore` are set via enums `Prop` and `AnyVal`
- `Entity` is the enum with variants of different entities
- `EntityList` instance is used in `Game` for storing all related to the entities within game `Id`s 
- `Game` that responsible for storing current `CoordSys` and `EntityList` and running related scripts
- `GameObject` that stands for basic game object
- `GameCamera` with four different constructors
- Reexports into scope of namespace `engine`

### Changed
- `linalg` module renamed to `linal`
- `enums` module rightfully renamed to `errs`

### Fixed
- `VecSpace` now knows precisely whether the basis is orthogonal or not

### Dependecies
- `Uuid` crate with feauture `v4`


## [0.0.2] - 2023-04-13

### Added
- Constructors for rotational matrices

### Changed
- Global singleton ```Gramm``` matrix renamed to ```Gram``` matrix
- ```Vecspace``` struct renamed to ```VecSpace``` struct

### Fixed
- No more panic in scalar product if operands are transposed incorrectly

### Removed
- Pointless operation of addition, subtraction between ```Matrixify``` implementor and number
- Enum ```MatrixType``` that had no usages

## [0.0.1] - 2023-03-30

### Added

- ```Matrixify``` trait that intended to be implemented for each struct
that may be represented as a table of numbers
- ```Matrix``` struct and Matrixify implementation for it
- ```Vector``` struct and Matrixify implementation for it
- Arithmetic operations such as addition, subtraсtion, multiplication,
division between ```Matrix``` and ```Vector```, between them two and numbers
- Scalar product between two ```Vectors``` in basis of vector space or without it
- Vector product between two ```Vectors```
- ```VecSpace``` struct contains basis
- ```Point``` struct as just another representation of Vector in basis
- ```CoordSys``` struct defined with ```VecSpace``` and initial ```Point```
- Global singletones: matrix of bilinear form, actual coordinate system,
corresponding Gram matrix