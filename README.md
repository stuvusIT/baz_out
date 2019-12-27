# baz_out

This is a little program that works together with
[castle](github.com/haslersn/castle).
baz_out locks the lock if there was no change for a certain period of time.

baz_out realizes this by accessing a castle `/lock` endpoint, where
the response contains a `last_change` JSON field which is the timestamp of the
last change.

## Build

```bash
$ cargo build
```

### With Nix

```bash
$ nix-build
```

With Nix you can also easily cross-compile baz_out:

```bash
$ nix-build '<nixpkgs>' \
    --arg crossSystem '{ config = "aarch64-unknown-linux-gnu"; }' \
    --arg overlays '[ (self: super: { baz_out = super.callPackage ./. {}; }) ]' \
    -A baz_out
```

## Configuration

In the working directory where baz_out is executed, there must be a
`baz_out.toml` configuration file.
A good start is to copy the `baz_out.toml.example` from this repository.

### [client] section

#### `endpoint =`

The `/lock` endpoint of the castle server.
It is used so as to `GET` the lock state and `PUT` lock requests.
Example: `"http://localhost:8020/castle/lock"`

### [policy] section

#### `lock_after_seconds =`

The number of seconds after which to lock the lock.
This is relative to the last change of the lock.
