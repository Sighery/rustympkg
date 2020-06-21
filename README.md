A _rusty_ (and very much incomplete) replacement of Arch Linux's `makepkg` and
similar tools.

**This doesn't yet implement actually building pacman packages.** I haven't
even gotten around to reading how that is done yet, and might just be too
complicated to ever bother.

For now all this tool can do is parse a given `PKGBUILD` (only certain keys,
as well), and print a JSON representation of it back into the `stdout`.

For usage, do:

```bash
rustympkg --help
```

This makes use of the [library crate `rustympkglib`][rustympkglib] that
implements most of the functionality, while this create just offers a binary
which provides a CLI with the use of `clap`.

### Near future

In the near future I plan on integrating `updpkgsums` into this binary (of
course most of the code would actually live in [rustympkglib][]).

Currently both `makepkg` and `updpkgsums` are too tightly coupled to Arch
Linux and don't seem to be able to be installed in other Linux distributions,
and updating `sums` arrays is integral for automating building Arch Linux
packages.

[rustympkglib]: https://github.com/Sighery/rustympkglib
