---
name: sync-languages
description: Sync language syllable files from the upstream Ruby project (folkengine/random_name_generator) into src/languages/, reporting new, changed, and unchanged files, and wiring any brand-new languages into the Language enum. Use when the user types /sync-languages or asks to update, refresh, or pull the latest language files from the Ruby project.
---

# Sync language files from the upstream Ruby project

The Ruby project <https://github.com/folkengine/random_name_generator> is the
root project and the source of truth for language syllable files. This Rust
port vendors copies under `src/languages/`, which are embedded into the binary
via `rust-embed` (`#[folder = "src/languages/"]` in `src/lib.rs`).

Goal: make `src/languages/` match the latest upstream content, and wire any
brand-new languages into the Rust code.

## Upstream layout and name mapping

Upstream keeps files in three places, with lowercase-kebab names. This repo
uses TitleCase filenames, and Cyrillic names for the `-ru` dialects. Apply
this mapping:

| Upstream path | Local file in `src/languages/` |
|---|---|
| `lib/languages/<name>.txt` | `<Name>.txt` (title-case the basename) |
| `lib/languages/experimental/<name>.txt` | `<Name>.txt` (same rule; the `experimental/` dir is flattened away) |
| `lib/languages/elven-ru.txt` | `Эльфийский.txt` |
| `lib/languages/fantasy-ru.txt` | `Фантазия.txt` |
| `lib/languages/goblin-ru.txt` | `Гоблин.txt` |
| `lib/languages/roman-ru.txt` | `Римский.txt` |
| `spec/languages/test-<name>.txt` | `Test-<name>.txt` (only sync ones that already exist locally, e.g. `Test-micro.txt`, `Test-tiny.txt`; other spec fixtures are Ruby-test-specific) |

If a new `-ru` (or other non-Latin-alphabet) dialect appears upstream that is
not in the table, do not guess a Cyrillic name — ask the user what to call it.

## Steps

1. **List upstream files** (no auth needed):

   ```sh
   curl -s "https://api.github.com/repos/folkengine/random_name_generator/git/trees/main?recursive=1"
   ```

   Collect every `.txt` path under `lib/languages/` and the `spec/languages/`
   test fixtures covered by the table above.

2. **Download each mapped file** to a scratch directory (not directly into the
   repo) from
   `https://raw.githubusercontent.com/folkengine/random_name_generator/main/<upstream path>`.

3. **Compare** each downloaded file against its counterpart in
   `src/languages/` and classify: **unchanged**, **changed** (diff the
   syllable lists), or **new** (no local counterpart). Copy changed and new
   files into `src/languages/`. Never delete a local file that upstream lacks
   without asking — local-only files may be intentional.

4. **Wire up brand-new languages.** A new `.txt` file alone is not reachable
   from the API. For each new language:
   - Add a variant to the `Language` enum in `src/lib.rs` (region
     `// region Language`). The variant name must match the filename stem
     exactly, since `Language::get_filename()` derives the filename from the
     `Debug` name via `format!("{self}.txt")`.
   - Decide with a quick look whether it belongs in the
     `Distribution<Language>` random sample in the same region (the main five
     languages are sampled; dialects and experimental ones are not).
   - Add a smoke test alongside the existing per-language tests in
     `src/lib.rs` (see `try_from__fantasy` and friends).
   - Check `src/main.rs` / clap flags: if languages are exposed as CLI
     switches, add the new one there too.

5. **Verify** with `make` (runs fmt, build, tests, doc tests, clippy
   pedantic with `-Dwarnings`, docs). Syllable files with characters outside
   the parser's regex will fail tests here — if a new file fails to parse,
   report the offending syllable lines rather than silently dropping the
   language.

6. **Report** a summary table: file, status (new / updated / unchanged), and
   any code changes made. Leave everything uncommitted — staging and commits
   are the user's job.
