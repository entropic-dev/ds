# ds/dstopic architecture

This document gives a super-high-level view of the various components of `ds`, the Entropic CLI.

It is, right now, only an initial sketch at an architectural design of the overall application, and is split into two halves: `ds`, the Rust client with the bulk of the package management logic; and `dstopic`, the Node script runner that takes advantage of the work `ds` does to provide a bunch of nice features.

## ds

`ds` is the primary interface for users. It's the actual Rust-based CLI, which runs all the various user-level commands. It wraps [`dstopic`](#dstopic), currently invoking it as a subprocess.

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

Implementation: A sketch was written up: https://github.com/npm/tink#--tink-check

#### <a name="ds-check"></a> `$ ds test`

Runs the test suite, including `ds check`.

#### <a name="ds-unwrap"></a> `$ ds unwrap [pkg...]`

Extracts a package into `node_modules/`, or all dependencies (and their dependencies) if an argument is passed. Only direct dependencies can be passed as arguments.

Implementation: This needs to invoke the installer, once it exists.

## dstopic

`dstopic` is the Node side of the package manager. It's a series of patches to Node that enable things like TypeScript support, shared package files, and other stuff.

### Node Patches

The idea behind `ds sh` is that it replaces `node` as your invoked binary, adding various features like not requiring a physical `node_modules/` directory, auto-fetching missing dependency files, etc. In order to achieve this, it loads node and monkey-patches `fs`, as well as adding some new extensions, and hooking up `spawn-wrap` so child processes replicate these patches.

`dstopic` will invoke `ds` as-needed to make sure all required packages are available.

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

- `.ts` - typescript loading, using the typescript compiler
- `.jsx`/`.tsx` - JSX loading, through babel
- `.wasm` - WebAssembly module support, through [`esm`](https://npm.im/esm)
- ESM support - through `esm` as well.

Implementation: Mostly copy tink here: https://github.com/npm/tink/blob/latest/lib/node/extensions.js

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

Implementation: This will be written using [surf](https://crates.io/crates/surf), and will use [surf-middleware-cache](https://github.com/zkat/surf-middleware-cache) once it's ready.

### cacache-rs

This is the Rust implementation of cacache, a content-addressable cache where all package metadata and individual files for Entropic are stored for fast, easy access. It deduplicates globally by content address, reducing overall storage use.
