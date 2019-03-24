src2bin
=======

[![asciicast](https://asciinema.org/a/236012.svg)](https://asciinema.org/a/236012)


about
-----

`bin2src` takes an input stream of bytes and converts
that into source code. That source code can then be 
easily embedded in future projects.


usage
-----

Read bytes from STDIN and write them to STDOUT:

    $ echo "hello" | bin2src
    \x68\x65\x6c\x6c\x6f\x0a

Note that there is no need to convert your input bytes to a plain text encoding:

    $ head -c 10 /dev/urandom | bin2src 
    \x30\xc7\x18\x67\x5b\xc3\x7b\x7d\x12\xcd

The primary purpose of `bin2src` is to facilitate embedding binary blobs into software source code.
To specify an output format, use the `-f` option ("f" for format):

    $ head -c 10 /dev/urandom | bin2src -f python
    DATA = """\x22\x82\x2d\xd4\xb8\x8c\x36\xb4\x35\x21"""

If you would like to change the variable name, use the `-a` option ("a" is a common mathematical variable):

    $ head -c 10 /dev/urandom | bin2src -f python -a RAND_BYTES
    RAND_BYTES = """\xf5\xde\x5e\xdc\x66\xeb\x89\x24\x13\xd5"""

Other options are available. See the full list by running `bin2src --help`. 

supported output formats
------------------------

- plaintext (default)
- go
- rust
- python

building
--------

The easiest route is via `git` and `cargo`:

    $ git clone https://git.nzoss.org.nz/tim-mcnamara/bin2src
    $ cd bin2src
    $ cargo build

`bin2src` will now be available within the `./target/debug` directory:

    $ echo Hello | ./target/debug/bin2src -f rust
    const DATA: &[u8] = b"\x48\x65\x6c\x6c\x6f\x0a";


contributing
------------

**Contributions welcome!**

Adding support for a new language is very easy. It should take no 
more than adding 10 lines of code. 

If you're not comfortable writing software, but still have an idea to
extend the project -- that's fine too. Please file an issue with the language that you would like
supported and/or any other features that you are 
interested in.


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
