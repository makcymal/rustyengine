### RustyEngine - Simple Console Game Engine written in Rust

`lib` contains game engine as module that has the following features:

- Console drawing with ASCI characters that can be specified
- Traits for materials that can be treaten as game entites, eg planes, ellipses, empty entities, that for example can keeps game state variables. Define yourself entities by implemeting provided traits.
- Traits for materials stores that can be processed during event handling
- Traits for events and event sustems as well as simple event queue
- Game object defined with the given implementation of provided traits and parameters from Conf that stands for configuration read from .toml file

`bin` contains example of game where you should escape from labyrinth right in your console!
