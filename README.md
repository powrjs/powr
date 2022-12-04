# the `powr` project

`powr` aims to be a javascript/typescript engine to power serverless functions over the web.
the javascript code gets compiled to remove as much cold start interpreting would cause as possible.

the project currently doesn't support anything other than calling `console.log` with a `string` argument. if you want to
test that, you can run the following:

```bash
$ git clone https://github.com/pwrjs/pwr --recursive
$ cd pwr/samples
$ cargo run --release c hello_world.ts
$ ./hello_world
```

I recommend adding `--jobs [number of jobs]` to the `cargo run` command to speed up compilation.

## usage

section of the output of `powr --help`:

```
Usage:
	powr [OPTIONS] [FILE]

Flags:
	-h, --help : Show help

Commands:
	c, compile : Compile a TypeScript/JavaScript file
```

and `powr compile --help`:

```
Description:
	Compile a TypeScript/JavaScript file

Usage:
	powr compile [FILE] [OPTIONS]

Flags:
	-e, --emmit-llvm  : Emmit LLVM IR
	-d, --dry-run     : Only emits the LLVM IR
	-h, --help        : Show help

```

`--emit--llvm` leaves an `.ll` file that corresponds to the `.ts` file.

`--dry-run` outputs what would be emitted to the `.ll` file and exits.

## license

currently, the project uses [SSPL-1.0](LICENSE). this license will be changed when the project gets to a more usable
state. because this license blocks lesser effort (for a lack of a better term!) contributions (and these are very
desirable!), the license may become more permissive before the project is shippable.
