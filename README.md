# the `power` project

`power` aims to be a javascript/typescript engine to power serverless functions over the web.
the javascript code gets compiled to remove as much cold start interpreting would cause as possible.

## roadmap

for this project to get on a usable state as fast as possible, it will have to follow a roadmap of changes.

### transpiling

the initial state of the project will be to transpile JS/TS code into another language.
abstracting the compilation side of the project to focus on getting everything else in place.
there's three potential languages `power` could compile to:

- C++
- Zig
- Rust

each of these languages would have their pros and cons into being used in this project. which is why eventually
they will have to be replaced to compiling directly instead.

## license

currently, the project uses [SSPL-1.0](LICENSE). this license will be changed when the project gets to a more usable state. because this license blocks lesser effort (for a lack of a better term!) contributions (and these are very desirable!), the license may become more permissive before the project is shippable.
