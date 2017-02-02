`countmerge` is a fast command-line program (written in Rust) that takes in a
tab-separated file that maps keys to counts, and adds up the total count for
each key.

It requires the input to be sorted by key, so you should probably run
`sort` on it first.

Essentially, it's `uniq` that can add.

It takes no options as of now. It just takes in lines on standard input and
writes the summed lines to standard output. Lines that don't map a key to an
integer count will be output as-is.

Example input:

	key A	1
    key A	2
    key B	3
    key C	4
    key	C	3
    key C	2
    key C	1

The output you get is:

	key A	3
    key B	3
    key C	10

## Installation

Install Rust 1.14 or later using [rustup](https://www.rustup.rs/).

Clone this repository, and run these commands in its directory:

    cargo build
    cargo install
