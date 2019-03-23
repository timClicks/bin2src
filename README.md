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

BIN2SRC is open source software. You're entitled to copy, store and redistribute 
BIN2SRC as provided under the terms of the [Blue Oak Model License][].

[Blue Oak Model License]: https://blueoakcouncil.org/license/1.0.0

```markdown
# Blue Oak Model License

Version 1.0.0

## Purpose

This license gives everyone as much permission to work with
this software as possible, while protecting contributors
from liability.

## Acceptance

In order to receive this license, you must agree to its
rules.  The rules of this license are both obligations
under that agreement and conditions to your license.
You must not do anything with this software that triggers
a rule that you cannot or will not follow.

## Copyright

Each contributor licenses you to do everything with this
software that would otherwise infringe that contributor's
copyright in it.

## Notices

You must ensure that everyone who gets a copy of
any part of this software from you, with or without
changes, also gets the text of this license or a link to
<https://blueoakcouncil.org/license/1.0.0>.

## Excuse

If anyone notifies you in writing that you have not
complied with [Notices](#notices), you can keep your
license by taking all practical steps to comply within 30
days after the notice.  If you do not do so, your license
ends immediately.

## Patent

Each contributor licenses you to do everything with this
software that would otherwise infringe any patent claims
they can license or become able to license.

## Reliability

No contributor can revoke this license.

## No Liability

***As far as the law allows, this software comes as is,
without any warranty or condition, and no contributor
will be liable to anyone for any damages related to this
software or this license, under any kind of legal claim.***
```

## trade mark

BIN2SRC is a trade mark of Tim McNamara.


## consumer protection

If you're using BIN2SRC for personal reasons, you have rights under New 
Zealand's Consumer Guarantees Act 1993. Be aware though that this 
software is software that you've downloaded from the Internet for 
free, so please use use your own diligence.


## jurisdiction

In case of any disputes, the laws of New Zealand apply.