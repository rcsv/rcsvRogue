# TOML
Tom's Obvious, Minimal Language by Tom Preston-Werner. Latest tagged version: [v.0.5.0](https://github.com/mojombo/toml/blob/master/versions/en/toml-v0.5.0.md).

NOTE: The `master` branch of this repository tracks the very latest development and may contain features and changes that do not exist on any released version. To find the spec for a specific version, look in the `versions` subdirectory.

As of version 0.5.0, TOML should be considered extremely stable. The goal is for version 0.1.0 to be backwards compatible (as much as humanly possible) with version 0.5.0. All implementations are strongly encouraged to become 0.5.0 compatible so that the transition to 1.0.0 will be simple when that happens.

## Objectives

TOML aims to be a minimal configuration file format that's easy to read due to obcious semantics. TOML is designed to map unambiguously to a hash table. TOML should be easy to parse into data structure in a wide variety of languages.

## Table fo contents
- [Example](#user-content-example)
- [Spec](#user-content-spec)
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

* TOML is case sensitive.
* A TOML file must be a valid UTF-8 encoded Unicode document.
* Whitespace means tab (0x09) or space (0x20).
* Newline means LF (0x0A) or CRLF (0x0D0A).

## Comment

A hash symbol marks the rest of the line as a comment.

```toml
# This is a full-line comment
key = "value" # This is a comment at the end of line
```

## Key/Value Pair
The primary building block of a TOML document is the key/value pair.

Keys are on the left of the equals sign and values are on the right. Whitespace is ignored around key names and values. The key, equals sign, and value must be on the same line (though some values can be broken over multiple lines).

```toml
key = "value"
```

Values must be of the following types: String, Integer, Float, Boolean, Datetime, Array, or Inline Table. Unspecified values are invalid.

```toml
key = # INVALID
```

## Keys

A key may be either bare, quoted, or dotted.

**Bare keys** may only contain ASCII letters, ASCII digits, underscore, and dashes (`A-Za-z0-9_-`). Note that bare keys are allowed to be composed of only ASCII digits, e.g. `1234`, but are always interpreted as strings.

```toml
key = "value"
bare_key = "value"
bare-key = "value"
1234 = "value"

**Quoted keys** follow the exact same rules as either basic strings or literal strings and allow you to use a much broader set of key names. Best practice is to use bare keys ecept when absolutely necessary.

```toml
"127.0.0.1" = "value"
"character encoding" = "value"
"ʎǝʞ" = "value"
'key2' = "value"
'quoted "value"' = "value"
```
A bare key must be non-empty, but an empty quoted key is allowed (though discouraged).

```toml
= "no key name"  # INVALID
"" = "blank"     # VALID but discouraged
'' = 'blank'     # VALID but discouraged
```

***Dotted keys** are a sequence of bare or quoted key joined with a dot. This allows for grouping similar properties together:

```toml
name "Orange"
physical.color = "orange"
phisical.shape = "round"
site."google.com" = true
```

In JSON land, that would give you the following structure:

```json
{
    "name": "Orange",
    "physical": {
        "color": "orange",
        "shape": "round"
    },
    "site": {
        "google.com": true
    }
}
````

whitespace around dot-separated parts is ignored, however, best practice is to not use any extraneous whitespace.

Defining a key multiple times is invalid.

```toml
# DO NOT DO THIS
name = "Tom"
name = "Pradyun"
```

As long as a key hasn't been directly defined, you may still write to it and to names within it.

```toml
a.b.c = 1
a.d   = 2
```

```toml
# THIS IS INVALID
a.b = 1
a.b.c = 2
```

## String
There are four ways to express strings: basic, multi-line basic, literal, and multi-line literal. All strings must contain only valid UTF-8 characters.

**Basic strings** are surrounded by quotation marks. Any Unicode character may be used except those that must be escaped: quotation mark, backslash, and the control characters (U+0000 to U+001F, U+007F).

```toml
str = "I'm a string. ¥"You can quote me¥". Name¥tJos¥u00E9¥nLocation¥tSF."
```

For convenience, some popular characters have a compact escape sequence.

```
¥b         - backspace       (U+0008)
¥t         - tabular         (U+0009)
¥n         - linefeed        (U+000A)
¥f         - form feed       (U+000C)
¥r         - carriage return (U+000D)
¥"         - quote           (U+0022)
¥¥         - backslash       (U+005C)
¥uXXXX     - unicode         (U+XXXX)
¥UXXXXXXXX - unicode         (U+XXXXXXXX)
```

Any Unicode character may be escaped with the `¥uXXXX` or `¥UXXXXXXXX` form. The escape codes must be valid Unicode [scalar values](http://unicode.org/glossary/#unicode_scalar_value).

All other escape sequence not listed above are reserved and, if used, TOML should produce an error. Sometimes you need to expres passages of text (e.g. translation files) or would like to break up a very long string into multiple lines. TOML makes this easy.

**Multi-line basic strings** are surrounded by three quotation marks on each
side and allow newlines. A newline immediately following the opening delimiter
will be trimmed. All other whitespace and newline characters remain intact.

```toml
str1 = """
Roses are red
Violets are blue"""
```

TOML parsers should feel free to normalize newline to whatever makes sense for
their platform.

```toml
# On a Unix system, the above multi-line string will most likely be the same as:
str2 = "Roses are red\nViolets are blue"

# On a Windows system, it will most likely be equivalent to:
str3 = "Roses are red\r\nViolets are blue"
```

For writing long strings without introducing extraneous whitespace, use a "line
ending backslash". When the last non-whitespace character on a line is a `\`, it
will be trimmed along with all whitespace (including newlines) up to the next
non-whitespace character or closing delimiter. All of the escape sequences that
are valid for basic strings are also valid for multi-line basic strings.

```toml
# The following strings are byte-for-byte equivalent:
str1 = "The quick brown fox jumps over the lazy dog."

str2 = """
The quick brown \


  fox jumps over \
    the lazy dog."""

str3 = """\
       The quick brown \
       fox jumps over \
       the lazy dog.\
       """
```

Any Unicode character may be used except those that must be escaped: backslash
and the control characters (U+0000 to U+001F, U+007F). Quotation marks need not
be escaped unless their presence would create a premature closing delimiter.

If you're a frequent specifier of Windows paths or regular expressions, then
having to escape backslashes quickly becomes tedious and error prone. To help,
TOML supports literal strings which do not allow escaping at all.

**Literal strings** are surrounded by single quotes. Like basic strings, they
must appear on a single line:

```toml
# What you see is what you get.
winpath  = 'C:\Users\nodejs\templates'
winpath2 = '\\ServerX\admin$\system32\'
quoted   = 'Tom "Dubs" Preston-Werner'
regex    = '<\i\c*\s*>'
```

Since there is no escaping, there is no way to write a single quote inside a
literal string enclosed by single quotes. Luckily, TOML supports a multi-line
version of literal strings that solves this problem.

**Multi-line literal strings** are surrounded by three single quotes on each
side and allow newlines. Like literal strings, there is no escaping whatsoever.
A newline immediately following the opening delimiter will be trimmed. All
other content between the delimiters is interpreted as-is without modification.

```toml
regex2 = '''I [dw]on't need \d{2} apples'''
lines  = '''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''
```

Control characters other than tab are not permitted in a literal string. Thus,
for binary data it is recommended that you use Base64 or another suitable ASCII
or UTF-8 encoding. The handling of that encoding will be application specific.


