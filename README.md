# Plantill
A personal project to use templates so that starting projects in languages that don't have a `cargo new` equivalent doesn't take so long (e.g. LaTeX or Common Lisp).

## Usage

### Config
Under `~/.config/templater/config.toml`.

Usage: Every template is a `table`. Each one has:

|  Name    |  Description                                        | Default                               |
|----------|-----------------------------------------------------|---------------------------------------|
| `source` | Directory under `~/.config/plantill/` to copy over  | `~/.config/plantill/[template name]`  |

### Templates
After adding the folder under `~/.config/plantill`

#### Example
Check the above section for each section's meaning

```toml
[lispt]
source = "lispt"

[LaTeX]
source = "latex"
```
