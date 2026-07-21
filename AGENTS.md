# Repository Agent Guide

## Aim

This is a learning repository for hand-coded mini-projects in Rust and C, with
Go planned for later. The owner knows each language's basics and wants projects
that combine multiple concepts without following the usual compiler, shell,
database, web-server, CRUD, or command-clone path.

The current difficulty is basic to early intermediate. Do not raise it, add a
language, or expand the catalog unless the owner explicitly asks.

## Source Of Truth

`README.md` is the project catalog and defines each MVP. Before implementing a
project, read its description and boundaries. Preserve those boundaries unless
the owner approves a scope change.

## Repository Layout

- Put Rust projects in `rust/<project-name>/`.
- Put C projects in `c/<project-name>/`.
- Keep every project independently buildable and testable.
- Keep project-specific source, tests, fixtures, build files, and documentation
  inside that project's directory.
- Do not create shared libraries, a root Cargo workspace, or a root build system
  until actual duplication justifies one.

## Project Limits

- Keep no more than three listed projects per language at the current level.
- Do not implement a project until the owner selects it.
- Finish and verify the MVP before proposing stretch goals.
- Rust and C should remain distinct learning tracks, not ports of one another.
- When a project is started or completed, update its status in `README.md`.
- `.research/project-idea-backlog.md` is a non-catalog idea pool, not part of
  the project catalog. Promote an idea only when the owner selects it and
  decides which catalog entry it replaces.

## Rust Guidance

- Use stable Rust and standard Cargo conventions.
- Prefer a small library/binary split: domain logic in `src/lib.rs`, CLI wiring
  in `src/main.rs`.
- Use mature parsing crates named by the project scope instead of rebuilding
  established formats. Hand-code the analysis and domain model.
- Model findings and errors with explicit types rather than free-form strings.
- Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`.

## C Guidance

- Target C17 unless a project documents another standard.
- Use a small, readable build file local to the project.
- Parse bytes explicitly; do not cast untrusted buffers to structs whose
  padding or alignment is implementation-defined.
- Check allocation, I/O, arithmetic, indexes, lengths, and integer conversions.
- Keep parsing, domain logic, and presentation separable enough to test.
- Compile with strict warnings and sanitizers during development where the
  platform supports them.

## Testing And Safety

- Treat all input files as untrusted and potentially truncated.
- Use tiny synthetic fixtures that expose boundary cases and malformed input.
- Add focused unit tests plus at least one end-to-end CLI test per project.
- Never commit generated binaries, build directories, large media files,
  captures containing private traffic, or fixtures with sensitive metadata.
- Keep changes minimal and explain non-obvious format or algorithm decisions.

## Research

The initial shortlist was selected from current online specifications,
ecosystem documentation, and project catalogs on 2026-07-21. Relevant links are
in `README.md`. Recheck primary specifications and current crate/tool behavior
before implementation rather than relying on memory or generic tutorials.
