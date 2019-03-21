# TOML
Tom's Obvious, Minimal Language by Tom Preston-Werner. Latest tagged version: [v.0.5.0](https://github.com/mojombo/toml/blob/master/versions/en/toml-v0.5.0.md).

NOTE: The `master` branch of this repository tracks the very latest development and may contain features and changes that do not exist on any released version. To find the spec for a specific version, look in the `versions` subdirectory.

As of version 0.5.0, TOML should be considered extremely stable. The goal is for version 0.1.0 to be backwards compatible (as much as humanly possible) with version 0.5.0. All implementations are strongly encouraged to become 0.5.0 compatible so that the transition to 1.0.0 will be simple when that happens.

## Objectives

TOML aims to be a minimal configuration file format that's easy to read due to obcious semantics. TOML is designed to map unambiguously to a hash table. TOML should be easy to parse into data structure in a wide variety of languages.

## Table fo contents
- [Example](#user-content-example)

## Example
```toml
# This is a TOML document.

title = "TOML Example

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates

[database]
server = "192.168.1.1"
ports = [ 8001, 8001, 8002 ]
connection_max = 5000
enabled = true

[servers]

  # Indentation (tabs and/or spaces) is allowed but not required
  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"

  [servers.beta]
  ip = "10.0.0.2"
  dc = "eqdc10"

[clients]
data = [ [ "gamma", "delta"], [1, 2] ]

# Line breaks are OK when inside arrays
hosts = [
  "alpha",
  "omega"
]
```
## Spec

