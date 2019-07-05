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

### Node Patches

The idea behind `ds` is that it replaces `node` as your invoked binary, adding various features like not requiring a physical `node_modules/` directory, auto-fetching missing dependency files, etc. In order to achieve this, it loads node and monkey-patches `fs`, as well as adding some new extensions, and hooking up `spawn-wrap` so child processes replicate these patches.

#### fs patches

This patches `fs` and makes it so anything going into `node_modules/` gets redirected over to the central cache, if there's not a physical version of a file at that path. It is unclear right now whether the file resolver this uses should call out to dstopic, or whether the resolver code should be duplicated in both languages for efficiency's sake.

#### spawn-wrap

This uses [`spawn-wrap`](https://npm.im/spawn-wrap) to make it so any node child processes also have the Node Patches installed.

#### extensions

This sets up all the [loaders](#loaders) for the various supported extensions, such as `.ts`, `.jsx`, and esm files.

### Loaders

This sets up various loaders for different kinds of source files:

* `.ts` - typescript loading, using the typescript compiler
* `.jsx`/`.tsx` - JSX loading, through babel
* `.wasm` - WebAssembly module support, through [`esm`](https://npm.im/esm)
* ESM support - through `esm` as well.

### Commands

These are the actual commands users run on the node-ish side.

#### <a name="ds-sh"></a> `$ ds sh [script.js]`

This is the replacement for `$ node`. It starts a node process with all the various [node patches](#node-patches) applied.

#### <a name="ds-prep"></a> `$ ds prep`

Pre-fetches metadata for required packages.

#### <a name="ds-add> `$ ds add [pkg...]`

Adds new dependencies. Edits Ds.toml. Does interactive search + add if no arguments passed to it.

#### <a name="ds-rm> `$ ds rm [pkg...]`

Removes existing dependencies. Edits Ds.toml. Goes interactive if no arguments passed ot it.

#### <a name="ds-up> `$ ds up [pkg...]`

Updates existing dependencies. Edits Ds.toml. Goes interactive if no arguments passed ot it.

#### <a name="ds-check"></a> `$ ds check`

Runs a bunch of checks, except the test suite.

#### <a name="ds-check"></a> `$ ds test`

Runs the test suite, including `ds check`.

#### <a name="ds-unwrap"></a> `$ ds unwrap [pkg...]`

Extracts a package into `node_modules/`, or all dependencies (and their dependencies) if an argument is passed. Only direct dependencies can be passed as arguments.

## dstopic (🦀)

`dstopic` is the Rust side of the package manager, and ultimately where the core of the package manager logic exists. It's its own CLI/Server and is wrapped by [`ds`](#ds) to provide a more Node-friendly frontend.

### file resolver

Resolves a path within `node_modules/` into a path into [`cacache-rs`](#cacache-rs). Fetches any missing package metadata as it resolves.

### ensure file

Downloads a specific file by content address from Entropic. Requires metadata that [`ensure package`](#ensure-package) fetches.

### ensure package

Makes sure everything needed for a package is available. When a package needs to actually be extracted to `node_modules/` in order to work, this does that. Otherwise, it just fetches metadata and lets individual file fetches be lazy, through [ensure file](#ensure-file).

### installer

This is the main interface into the "installer" part of the package manager. It contains all the installation logic. (handwave. This needs more fleshing out.)

### tree builder

This is the part of the [installer](#installer) that actually takes care of calculating the target tree. It fetches any missing metadata using the [entropic client](#entropic-client) and builds a data structure representing the final dependency tree.

### script runner

This runs any required run-scripts. By default, it will prompt if a new package wants to run an install script, and save its result to the lockfile.

### cacache-rs

This is the Rust implementation of cacache, a content-addressable cache where all package metadata and individual files for Entropic are stored for fast, easy access. It deduplicates globally by content address, reducing overall storage use.
