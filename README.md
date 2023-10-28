# Plantill
A personal project to use templates so that starting projects in languages that don't have a `cargo new` equivalent doesn't take so long (e.g. LaTeX or Common Lisp).

## Instalation
Git clone, compile it and add the binary (`target/release/plantill`) to `$PATH`. For ease, there's
an included `justfile` that compiles it (`cargo build --release`) and adds it to `$PATH` by 
copying it to `~/.local/bin` to use it:
```sh
git clone [url]
just copy 
``` 

It may be done manually as well, it's just a binary that should be in `$PATH` for 
convenience (or, hell, you can even alias to it).

## Usage
The basics are simple: the config (`config.toml`) and templates (directories next to `config.toml`) are to be stored under 
`~/.config/plantill` (this path is hard-coded and is, as-of-now, not modifiable without recompiling the binary).

Then, in some directory, run the binary "`plantill`", select the template and a project name, and the template
will be copied over with all instances of "`plantillname`" and "`PLANTILLNAME`" replaced with the project name in
lower and upper case accordingly. 

## Config
Under `~/.config/templater/config.toml`.
Usage: Every template is a `table`. Each one has:

|  Name                 |  Description                                                                         |
|-----------------------|--------------------------------------------------------------------------------------|
| `source`              | Directory under `~/.config/plantill/` to copy over                                   |
| `should_replace_name` | Toggles the replacement of `plantillname` and `PLANTILLNAME` with the project's name |


### Example
Check the above section for each line's meaning

```toml
[lispt]
source = "lispt"
should_replace_name = true

[LaTeX]
source = "latex"
should_replace_name = false
```

Example of `~/.config/plantill/`
```text
./
|
+-- config.toml
+-- lispt/  <-- a template listed as a source
|    |-- Some file
|    |-- Some other file
|      
+-- latex/  <-- another template also listed as a source
|    |-- Some some file
|    |-- Some some other file
```

## Roadmap

- [x] Parse toml, extract templates
- [x] Ask for which to use
- [x] Ask for project name
- [x] Verify that selected template folder exists (implicit)
- [x] Try to copy it over to current location
- [x] Replace corresponding keys with project name
 - [x] "PLANTILLNAME" for the upper name
     - [ ] In file names (deemed unnecessary, is trivial to add)
     - [x] Inside files
 - [x] "plantillname" for the lower name
     - [x] In file names
     - [x] Inside files
