# ds/dstopic architecture

This document gives a super-high-level view of the various components of `ds`, the Entropic CLI.

It is, right now, only an initial sketch at an architectural design of the overall application, and is split into two halves: `dstopic`, the Rust client with the bulk of the package management logic; and `ds`, the Node-based frontend for that package manager.

## Diagram

This diagram is the larger architecture and how components connect:

```
╔═════════╦════════════════════════════════════════════════════════════════════╗
║ ds (⬡) ║                                                                    ║
╠═════════╝    ┌───────┬ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                                 ║
║              │loaders│┌───┐                 │                                ║
║              ├───────┘│esm│─┐                                                ║
║                       └───┘ │    ┌───┐  ┌──┐│                                ║
║              │          ┌──────┐ │jsx│  │ts│                                 ║
║                ┌────┐ ┌─│esm.js│ └───┘  └──┘│                                ║
║              │ │wasm│─┘ └──────┘   ▲      ▲                                  ║
║                └────┘       ▲      │      │ │                                ║
║              └ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ┼ ─ ─ ─│─                                 ║
║                        ┌────┴──────┴──────┘                                  ║
║                        │                ┌────────┐─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐  ║
║┌──────────────┐─ ─ ─ ─ ┼ ─ ─ ─ ─ ┐      │commands│   ┌────────┐┌─────────┐   ║
║│ node patches │        │                ├────────┘ ┌─┤$ ds add││$ ds srch││  ║
║├──────────────┘  ┌──────────┐    │        ┌───────┐│ └────┬───┘└─────────┘   ║
║        ▲         │extensions│        ┌──┼─│$ ds sh├┘ ┌────┴───┐┌────────┐ │  ║
║│       │         └──────────┘    │   │    └───┬───┘  │$ ds rm ││$ ds ck │    ║
║        ▼                          ◀──┘  │┌────┴────┐ └────┬───┘└────────┘ │  ║
║│┌────────────┐      ┌────┐       │       │$ ds prep│ ┌────┴───┐┌───────┐     ║
║ │ spawn-wrap │      │ fs │              │└─────────┴─┤$ ds up ││$ ds uw│  │  ║
║│└────────────┘      └────┘       │                   └────────┘└───────┘     ║
║ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─║─ ─ ─ ─ ─       └ ─ ─ ─ ─ ─ ─ ─ ─ ╦ ─ ─ ─ ─ ─ ─ ─ ┘  ║
╚════════════════════════╬══════════════════════════════════╬══════════════════╝
                         ║                                  ║
                         ║                                  ║
                         ║           ┌───────┐              ║
                         ╚═══╗       │  IPC  │              ║
                             ║       └───────┘      ╔═══════╝
                             ║                      ║
                             ║                      ║
╔══════════════╦═════════════╬══════════════════════╬══════════════════════════╗
║ dstopic (🦀) ║             ▼                      ║                          ║
╠══════════════╝      ┌─────────────┐               ║                          ║
║           ┌─────────│file resolver│◀───┐          ║         ┌───────────────┐║
║           │         └─────────────┘    │          ▼       ┌▶│ tree resolver │║
║           │                            └────┌───────────┐─┘ └───────────────┘║
║           ▼                                 │ installer │                    ║
║     ┌───────────┐               ┌──────────▶└───────────┘─┐  ┌─────────────┐ ║
║     │ensure-file│───┐           │                 │       └─▶│script-runner│ ║
║     └───────────┘   └──▶┌──────────────┐          │          └─────────────┘ ║
║           │             │ensure-package│          │                          ║
║           │         ┌───└──────────────┘          │                          ║
║           │         │                             │                          ║
║           │         ▼                             ▼                          ║
║           │  ┌────────────┐               ┌───────────────┐                  ║
║           └─▶│ cacache-rs │◀──────────────│entropic-client│                  ║
║              └────────────┘               └───────────────┘                  ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

## ds (⬡)

`ds` is the primary interface for users. It's the actual Node-based CLI, which runs all the various user-level commands. It wraps [`dstopic`](#dstopic), currently invoking it as a subprocess, for all its package-management work.

Implementation: This is all written in Node, and kinda has to be, because it's wrapping it to add additional runtime behavior.

### Node Patches

The idea behind `ds` is that it replaces `node` as your invoked binary, adding various features like not requiring a physical `node_modules/` directory, auto-fetching missing dependency files, etc. In order to achieve this, it loads node and monkey-patches `fs`, as well as adding some new extensions, and hooking up `spawn-wrap` so child processes replicate these patches.

#### fs patches

This patches `fs` and makes it so anything going into `node_modules/` gets redirected over to the central cache, if there's not a physical version of a file at that path. It is unclear right now whether the file resolver this uses should call out to dstopic, or whether the resolver code should be duplicated in both languages for efficiency's sake.

Implementation: This exists as an external module already, extracted from tink. See https://github.com/npm/fallback-fs

#### spawn-wrap

This uses [`spawn-wrap`](https://npm.im/spawn-wrap) to make it so any node child processes also have the Node Patches installed.

Implementation: Basically copy what tink is doing: https://github.com/npm/tink/blob/latest/lib/node/child_process.js

#### extensions

This sets up all the [loaders](#loaders) for the various supported extensions, such as `.ts`, `.jsx`, and esm files.

Implementation: Mostly copy tink here: https://github.com/npm/tink/blob/latest/lib/node/extensions.js

### Loaders

This sets up various loaders for different kinds of source files:

* `.ts` - typescript loading, using the typescript compiler
* `.jsx`/`.tsx` - JSX loading, through babel
* `.wasm` - WebAssembly module support, through [`esm`](https://npm.im/esm)
* ESM support - through `esm` as well.

Implementation: Mostly copy tink here: https://github.com/npm/tink/blob/latest/lib/node/extensions.js

### Commands

These are the actual commands users run on the node-ish side.

Implementation: I'm thinking the commands should be written on top of [yargs](https://npm.im/yargs), although yargs has the disadvantage that it can be a bit heavy to load sometimes, so some optimization work might be needed. Tink already has a useful architecture for how to structure these, but I'm open to alternative ideas if anyone wants to sketch them out.

#### <a name="ds-sh"></a> `$ ds sh [script.js]`

This is the replacement for `$ node`. It starts a node process with all the various [node patches](#node-patches) applied.

Implementation: https://github.com/npm/tink/blob/latest/lib/commands/shell.js

#### <a name="ds-prep"></a> `$ ds prep`

Pre-fetches metadata for required packages.

Implementation: This needs to invoke the installer, once it exists.

#### <a name="ds-add> `$ ds add [pkg...]`

Adds new dependencies. Edits Ds.toml. Does interactive search + add if no arguments passed to it.

Implementation: This needs to invoke the installer, once it exists. The interactive and UI behavior should be implemented using [ink](https://npm.im/ink).

#### <a name="ds-rm> `$ ds rm [pkg...]`

Removes existing dependencies. Edits Ds.toml. Goes interactive if no arguments passed ot it.

Implementation: This needs to invoke the installer, once it exists. The interactive and UI behavior should be implemented using [ink](https://npm.im/ink).

#### <a name="ds-up> `$ ds up [pkg...]`

Updates existing dependencies. Edits Ds.toml. Goes interactive if no arguments passed ot it.

Implementation: This needs to invoke the installer, once it exists. The interactive and UI behavior should be implemented using [ink](https://npm.im/ink).

#### <a name="ds-check"></a> `$ ds check`

Runs a bunch of checks, except the test suite.

Implementation: A sketch was written up: https://github.com/npm/tink#--tink-check

#### <a name="ds-check"></a> `$ ds test`

Runs the test suite, including `ds check`.

#### <a name="ds-unwrap"></a> `$ ds unwrap [pkg...]`

Extracts a package into `node_modules/`, or all dependencies (and their dependencies) if an argument is passed. Only direct dependencies can be passed as arguments.

Implementation: This needs to invoke the installer, once it exists.

## dstopic (🦀)

`dstopic` is the Rust side of the package manager, and ultimately where the core of the package manager logic exists. It's its own CLI/Server and is wrapped by [`ds`](#ds) to provide a more Node-friendly frontend.

Implementation: This is all written in Rust, and works as a standalone binary that will be called as a child process. Possibly, it will also have a long-running server + line protocol (or some other protocol) so the `ds` client doesn't need to constantly be spawning child processes, which is very heavy in some cases, such as with the [file resolver](#file-resolver).

### file resolver

Resolves a path within `node_modules/` into a path into [`cacache-rs`](#cacache-rs). Fetches any missing package metadata as it resolves.

Implementation: This should be ported to Rust: https://github.com/npm/tink/blob/latest/lib/pkglock.js#L22, and adapted accordingly.

### ensure file

Downloads a specific file by content address from Entropic. Requires metadata that [`ensure package`](#ensure-package) fetches.

Implementation: This takes a resolved file path into a package and downloads it using the [entropic client](#entropic-client), into the [cache](#cacache-rs).

### ensure package

Makes sure everything needed for a package is available. When a package needs to actually be extracted to `node_modules/` in order to work, this does that. Otherwise, it just fetches metadata and lets individual file fetches be lazy, through [ensure file](#ensure-file).

Implementation: When the [file resolver](#file-resolver) can't find metadata for a particular package, this is called and uses the [entropic client](#entropic-client) to download the appropriate package metadata, cache it, and return it so resolution can continue. This needs to be a blocking operation if invoked by the file resolver, and an async operation when invoked by the [installer](#installer)/[tree builder](#tree-builder).

### installer

This is the main interface into the "installer" part of the package manager. It contains all the installation logic. (handwave. This needs more fleshing out.)

Implementation: (handwaves). This is actually a much larger and more nuanced project, and this is just a placeholder.

### tree builder

This is the part of the [installer](#installer) that actually takes care of calculating the target tree. It fetches any missing metadata using the [entropic client](#entropic-client) and builds a data structure representing the final dependency tree.

Implementation: This is a larger project, but a good starting point would be to prototype the necessary tree data structure and any necessary manipulation methods on that that the installer will need.

### script runner

This runs any required run-scripts. By default, it will prompt if a new package wants to run an install script, and save its result to the lockfile.

Implementation: This should be a rewrite of https://github.com/npm/npm-lifecycle

### entropic client

This is an http client especially designed to communicate with the entropic registry. For now, this can be any client that supports both sync and async http requests. Long-term, it would be good to have built-in caching support (into [cacache](#cacache-rs)) that's fully http cache spec-compliant.

Implementation: The initial version of this can just be a straightforward wrapper around something like [`reqwest`](https://crates.io/crates/reqwest). It'll still need to manually write to cacache, though, for completed requests. It's completely ok for this client to load the entire request body into memory at one time, especially because of the state of async streams in Rust right now. It's also ok to limit http caching to the 5 minutes currently configured -- but it'll need to be able to do 304 requests, and that needs to be written manually. Long-term, a more complete port of [`make-fetch-happen`](https://npm.im/make-fetch-happen) (for retries, caching, keepalive, etc support), and [`npm-registry-fetch`](https://npm.im/npm-registry-fetch) (higher-level requests, standardized config consumption, etc) is desirable, but that's a much bigger effort than we need to undertake right now.

### cacache-rs

This is the Rust implementation of cacache, a content-addressable cache where all package metadata and individual files for Entropic are stored for fast, easy access. It deduplicates globally by content address, reducing overall storage use.

Implementation: Sync version is [already written](https://crates.io/crates/cacache), and an async API is currently in progress (privately). It is fully compatible with [`cacache-js`](https://npm.im/cacache), and needs to stay that way so [`ds`](#ds) is able to consume it directly if needed.
