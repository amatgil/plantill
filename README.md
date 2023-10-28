# Plantill
A personal project to use templates so that starting projects in languages that don't have a `cargo new` equivalent doesn't take so long (e.g. LaTeX or Common Lisp).

## Usage

### Config
Under `~/.config/templater/config.toml`.

Usage: Every template is a `table`. Each one has:

|  Name                 |  Description                                                                          |
|-----------------------|---------------------------------------------------------------------------------------|
| `source`              | Directory under `~/.config/plantill/` to copy over                                    |
| `should_replace_name` | Toggles the replacement of `plantillname` and `PLANTILLNAME` with the project's name" |

### Templates
After adding the folder under `~/.config/plantill`

#### Example
Check the above section for each line's meaning

```toml
[lispt] # Table name, template name
source = "lispt"

[LaTeX]
source = "latex"
```

## Roadmap

- [x] Parse toml, extract templates
- [x] Ask for which to use
- [x] Ask for project name
- [x] Verify that selected template folder exists (implicit)
- [x] Try to copy it over to current location
- [x] Replace corresponding keys with project name
 - [x] "PLANTILLNAME" for the upper name
     - [ ] In file names
     - [x] Inside files
 - [x] "plantillname" for the lower name
     - [x] In file names
     - [x] Inside files
