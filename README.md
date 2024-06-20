# [![Rust + \[Obj-C\]](assets/logo-small.png)](https://github.com/madsmtm/objc2) <br> Objective-C in Rust

[![License](https://badgen.net/badge/license/MIT/blue)](./LICENSE.txt)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

The crates you're interested in is probably:
- [`objc2`], which provides bi-directional interop between Rust and
  Objective-C, including support for defining Objective-C classes in Rust.
- [`objc2-*`] crates, which provides an autogenerated interfaces to Apple's
  Objective-C frameworks (`AppKit`, `Foundation`, `Metal`, `WebKit`, you name
  it, we [aim to have it](https://github.com/madsmtm/objc2/issues/393)).
- [`block2`], which provides bindings for Apple's C blocks, the
  C-equivalent of a Rust closure.

[`objc2`]: ./crates/objc2
[`objc2-*`]: ./framework-crates
[`block2`]: ./crates/block2


## Contact Us

Always feel free to [open an issue on GitHub](https://github.com/madsmtm/objc2/issues/new/choose).

If you prefer to have a more synchronous and less "formal" discussion, we have [a Matrix workspace](https://matrix.to/#/#objc2:matrix.org), feel free to ask any questions in the "Users" room.


## Goals

There are many conflicting priorities in an open-source project like this
(performance, ergonomics, understandability, portability, ...), but the
following two can be seen as the guiding principles for everything else in
this project.


### 1. Complete Soundness

The non-negotiable goal of these crates is to be completely "sound", meaning
it **must not be possible for safe Rust to cause undefined behaviour**!

As of January 2023, I (`@madsmtm`) have yet to find a single Rust crate or
project calling Objective-C soundly. Issues I have found include:
- Wrong method calling ABI.
- Wrong memory management (in the best cases they just leak a lot).
- Incorrect usage of `&mut`.
- Incorrect main-thread safety.

I don't state this to throw shade at these projects, it is very much
understandable! Objective-C and Rust have vastly different semantics, so
tackling these issues require vigilance and a focus that e.g. a system
clipboard crate just doesn't have!
Rather, I state it to provide reassurance: Rust's fearless concurrency can be
won back! This project's [approach to the issue][layered-safety] _works_,
and leaks, segfaults, race conditions and so on _can_ be completely
eliminated!

Needless to say, nothing is perfect, so if you think you've found a soundness
hole, please don't hesitate to report it on the [issue tracker]. Known
soundness holes (however theoretical) are tracked in the [`I-unsound`] label.

[layered-safety]: ./crates/objc2/src/topics/layered_safety.md
[`I-unsound`]: https://github.com/madsmtm/objc2/labels/I-unsound
[issue tracker]: https://github.com/madsmtm/objc2/issues/new


### 2. Idiomatic Rust

Soundness would be easy to achieve if we just marked every API as `unsafe`,
and called it a day (the precursor to this, `objc`, is basically sound).
However, that just pushes the burden onto you, the user, and then we're not
much better off!

As such, we'll try to be as safe and idiomatic as possible; using references
instead of pointers to represent objects and their mutability, `Option`
instead of `null`, doing memory management automatically instead of manually,
and so on (see again [these notes on "Layered Safety"][layered-safety]). These
abstractions should ideally be zero-cost, but this is of course a balancing
act against being ergonomic.

Some APIs in `objc2` and `block2` will still have to remain `unsafe`, so these
contain thorough `# Safety` sections, to let you know exactly which safety
guarantees you need to uphold.
The framework crates are a bit difficult in this regard, since they are mostly
autogenerated, which means that almost nothing can be safe by default!
However, we can still try to mitigate this problem by marking manually audited
functionality as safe (which we [need your help with][header-data]).

[header-data]: ./crates/header-translator/README.md


## Minimum Supported Rust Version (MSRV)

The _currently_ minimum supported Rust version is `1.60`; this is _not_
defined by policy, though, so it may change in at any time in a patch release.

Help us define a policy over in [#203].

[#203]: https://github.com/madsmtm/objc2/issues/203


## Migrating from `objc` and family

If size of your project is fairly small, it'll probably be easiest to just
jump straight into replacing everything to use the framework crates (when you
do, don't forget to [mark `unsafe` methods that you use as safe][header-data]
along the way).

If your project is large, you can consider upgrading in small steps, following
the changelog at each step of the way. For the most common cases, the
changelogs will include a helpful example on how to upgrade.

As an example you'd start by using `objc2` instead of `objc` in your
`Cargo.toml`:
```toml
[dependencies]
objc = { package = "objc2", version = "0.2.7" }
```

Afterwards, you can upgrade to the next release, in this case
`v0.3.0-alpha.0`, and make the required changes to your code following the
changelog. And so on, with every following release.


## License

This project is licensed under the MIT license, see [`LICENSE.txt`].

Work is in progress to make it dual-licensed under the Apache License
(Version 2.0) as well, see [this][#23].

[`LICENSE.txt`]: https://github.com/madsmtm/objc2/blob/master/LICENSE.txt
[#23]: https://github.com/madsmtm/objc2/issues/23


## Acknowledgements / Prior art

This repository is a merge of the following projects, see reasoning for the
fork [here](https://github.com/SSheldon/rust-objc/issues/101):
- [`objc`](https://github.com/SSheldon/rust-objc)
  - Renamed to `objc2`.
- [`objc-encode`](https://github.com/SSheldon/rust-objc-encode)
  - Renamed to `objc2-encode`.
- [`objc_exception`](https://github.com/SSheldon/rust-objc-exception)
  - Moved to `objc2::exception`.
- [`objc_id`](https://github.com/SSheldon/rust-objc-id)
  - Moved to `objc2::rc`.
- [`objc-foundation`](https://github.com/SSheldon/rust-objc-foundation)
  - Renamed to `objc2-foundation`.
- [`block`](https://github.com/SSheldon/rust-block)
  - Renamed to `block2`.

These were created almost solely by [@SSheldon](https://github.com/SSheldon),
so a huge thanks for their fantastic work on these crates!

This project also draws inspiration from:
- [`apple-sys`](https://github.com/youknowone/apple-sys)
- [`cacao`](https://github.com/ryanmcgrath/cacao)
- [the `core-foundation-rs` project](https://github.com/servo/core-foundation-rs)
- [`fruity`](https://github.com/nvzqz/fruity)
- [`metal`](https://github.com/gfx-rs/metal-rs)
- [`objrs`](https://gitlab.com/objrs/objrs)
- [`objr` and family](https://github.com/drewcrawford/objr#objr-expanded-universe)
- [`rust-macios`](https://github.com/a-isaiahharvey/rust-macios)
- [`uikit-sys`](https://github.com/simlay/uikit-sys) and `@simlay`'s [Objective-C work on `bindgen`](https://rust-lang.github.io/rust-bindgen/objc.html)
- [`cidre`](https://github.com/yury/cidre)

Finally, this is by far not the only project that ever tried to interoperate with Objective-C; other languages have done so as well (to varying degrees of success):
- Swift: Built from the beginning for Objective-C interop, and is what `objc2` aspires to have feature-parity with (though will probably never reach). Truly beautifully designed language!
- C#: Xamarin, [Xamarin.Mac](https://www.mono-project.com/docs/tools+libraries/libraries/monomac/), a good source of inspiration for what "should" work.
- Python: [PyObjC](https://pypi.org/project/pyobjc/) (previously?) official Apple project that worked with "BridgeSupport", nowadays they also [generate metadata by invoking Clang](https://github.com/ronaldoussoren/objective.metadata). Others include [`objp`](https://pypi.org/project/objp/) and [rubicon.objc](https://rubicon-objc.readthedocs.io/en/latest/index.html)
- Ruby: [MacRuby](http://macruby.org/), RubyCocoa
- Dart: [`ffigen`](https://github.com/dart-lang/ffigen/tree/master/example/objective_c)
- Kotlin: [somewhat built-in support](https://kotlinlang.org/docs/native-objc-interop.html)
- Nim: [somewhat built-in support](https://nim-lang.org/docs/backends.html), [`darwin`](https://github.com/yglukhov/darwin), [`objc`](https://github.com/jangko/objc)
- D: [somewhat built-in support](https://dlang.org/spec/objc_interface.html), [`derelict`](https://github.com/AuburnSounds/Dplug/tree/v12.8.0/macos/derelict/cocoa)
- Java: [Java-Objective-C-Bridge](https://github.com/shannah/Java-Objective-C-Bridge), [Multi-OS Engine: Nat/J](https://github.com/multi-os-engine/moe-natj) (also has a [generator](https://github.com/multi-os-engine/moe-natjgen)), Apple also has a very old official project.
- Node.js: [NodObjC](https://github.com/TooTallNate/NodObjC), [`objc`](https://github.com/lukaskollmer/objc)
- Zig: [zig-objcrt](https://github.com/hazeycode/zig-objcrt)
- D: [somewhat built-in support](https://dlang.org/spec/objc_interface.html), [`derelict`](https://github.com/AuburnSounds/Dplug/tree/v12.8.0/macos/derelict/cocoa)
- V: Not really existing, they just write and compile Objective-C code, and use manual C-bindings.
- Go: [MacDriver](https://github.com/progrium/macdriver)
