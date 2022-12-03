# the `power` project

`power` aims to be a javascript/typescript engine to power serverless functions over the web.
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

## license

currently, the project uses [SSPL-1.0](LICENSE). this license will be changed when the project gets to a more usable
state. because this license blocks lesser effort (for a lack of a better term!) contributions (and these are very
desirable!), the license may become more permissive before the project is shippable.
