# Chisel Changelog

All the most recent changes are summarised and grouped below:

## [0.1.0] - 2023-05-08

### Features

- Basic pretty printing implemented

### Miscellaneous Tasks

- Changelog generated

### Work In Progress

- Removed unused import
- Added some additional structures for simple tests
- Added some tasty, tasty fixtures
- Fleshing out the basic pretty print functionality
- Nightly commit
- Wip: removed `Action` specific result type and replaced with global
`ChiselResult` type

- Nightly commit
- Formatting changes on imports
- Further tidying and structuring
- Added placeholder for themeing
- Doc comments
- Fn name correction
- Adopted command nomenclature throughout
- Wip: various refactorings (adoption of command nomenclature throughout
the rendering pipeline bits)

- Started pissing about with crossterm
- Wip: added specific crossterm feature to allow for pluggable TUI backend
at a later date if required

- Wip: refactoring and further organisation of app state and background
rendering thread

- Basic machinery around rendering in a separate UI thread in place.
- Changed integer types to u16 where appropriate
- Changed term lib dependency to crossterm
- Changed changelog ordering to newest first
- Altered default cliff changelog intro
- Added cliff.toml


