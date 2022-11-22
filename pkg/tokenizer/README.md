# `power-tokenizer`

this is both a cli and a library.

tokenizes a string of characters. based on ECMAScript.

## installing

assumes you have cargo/rustup installed.

```shell
cargo install power-tokenizer
```

## running

```shell
power-tokenizer "const one_plus_one = 1 + 1"
```

expected output:

```
Const
Identifier(
    [
        'o',
        'n',
        'e',
        '_',
        'p',
        'l',
        'u',
        's',
        '_',
        'o',
        'n',
        'e',
    ],
)
Assign
Identifier(
    [
        '1',
    ],
)
Plus
Identifier(
    [
        '1',
    ],
)
EndOfFile

```