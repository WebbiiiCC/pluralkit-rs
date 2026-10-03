[![Crates.io](https://img.shields.io/crates/v/pluralkit.svg)](https://crates.io/crates/pluralkit)
[![License](https://img.shields.io/crates/l/pluralkit)](LICENSE)


# PluralKit Api Client

> This rust library provides a client for interacting with the Web Api of [PluralKit](https://pluralkit.me).  
> The client is fully async and backed by [reqwest](https://crates.io/crates/reqwest).

## Features

Note that each group of endpoints, as specified by [the documentation](https://pluralkit.me/api/endpoints), has it's own feature.  
The `systems`, `members`, `messages` features are enabled by default, so make sure to enable any additional features or disable the default features as you require.  
**Note:** You will not have access to privacy information without the `privacy` feature.
**Note:** If the `chrono` feature is disabled all timestamps are still available in raw string format.

## License

Licensed under the [MIT license](https://github.com/Web-44/simplyplural-rs/blob/master/LICENSE)