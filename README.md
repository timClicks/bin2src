src2bin
=======

about
-----

`bin2src` takes an input stream of bytes and converts
that into source code. That source code can then be 
easily embedded in future projects. 

It's intended to be a bin2c clone, but more general 
because it will support more than the C language.
At this stage though, only Rust language output is 
supported.


usage
-----

Read from STDIN and write to STDOUT:

    cat input.dat | bin2src


building
--------

The easiest route is via `git` and `cargo`:

    $ git clone https://git.nzoss.org.nz/tim-mcnamara/bin2src
    $ cd bin2src
    $ cargo build

`bin2src` will now be available within the `./target/debug` directory:

    $ echo Hello | ./target/debug/bin2src
    const DATA: &[u8] = b"\x48\x65\x6C\x6C\x6F\x0A";

From the `bin2src` source directory, it's also possible to ask 
`cargo` to figure out where the executable lives directly: 

    $ echo Hello | cargo run
       Compiling bin2src v0.1.0 (/home/tsm/Code/bin2src)
        Finished dev [unoptimized + debuginfo] target(s) in 0.35s
         Running `target/debug/bin2src`
    const DATA: &[u8] = b"\x48\x65\x6C\x6C\x6F\x0A";


contributing
------------

Please file an issue with the language that you would like
supported and/or any other contributions that you are 
interested in. When the project ready to accept contributions, 
these will be the first ones to be addressed.


legal
-----

## copyright

    Copyright 2019 Tim McNamara <code@timmcnamara.co.nz>

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
   limitations under the License.


## trade mark

BIN2SRC is a trade mark of Tim McNamara.


## consumer protection

Notwithstanding the disclaimer the above, you have rights under New 
Zealand's Consumer Guarantees Act 1993. Be aware though that this 
software is software that you've downloaded from the Internet for 
free, so please use use diligence.


## jurisdiction

In case of any disputes, the laws of New Zealand apply.