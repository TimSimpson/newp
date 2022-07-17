Project templates are stored in here. `build.rs`, executed by Cargo automatically, reads this directory and creates string literals used by the Rust code.

Mostly the project templates are straightforward: every template file in here is rendered as it is shown by [Tera](https://tera.netlify.app).

However, there are two special rules for filenames:

* filenames ending with `.tmpl` have this prefix removed when rendered
* filenames with some screamy-case names (such as `NAME`) are replaced with snake case equivalents surrounded by curly bases (ie `SNAKE_CASE_NAME` transfomrs to `{{ snake_case_name }}` (see below for a list)..

In both cases, this happens when `build.rs` reads the files and turns them into Rust code. So the non-generated Rust code already sees a filename such as `NAME_info.md.tmpl` as `{{NAME}}_info.md`.

## Template Context Variables

Here are the special variables passed to Tera when rendering the template files:

* author - the author (comes from a command line argument of the same name)
* camel_case_name - `name`, but in camelCase
* description - describes the project (comes from a command line argument of the same name)
* description_quoted - the description, but fully quoted so it can be dropped into languages such as JavaScript, C++, etc etc
* kebab_case_name - `name`, but in kebab-case
* license - the license (comes from a command line argument of the same name)
* name - the name of the project, passed in by the user
* pascal_case_name - `name`, but in PascalCase
* scream_case_name - `name`, but in SCREAM_CASE
* snake_case_name - `name`, but in snake_case
