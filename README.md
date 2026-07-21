# Pandora's Box

Handcrafted systems projects, because tutorials were getting too comfortable.

This repository is a learning lab for small but substantial programs written
from scratch in Rust and C. Each project combines several fundamentals into a
useful tool without trying to rebuild an entire compiler, database, shell, or
other well-worn wheel.

The current level is **basic to early intermediate**: language basics are
assumed, but every project has a deliberately narrow first version.

## Project Map

Projects are listed in a suggested learning order. `Planned` means the scope is
researched, but no implementation has started.

### Rust

Rust projects focus on domain modeling, safe stateful analysis, iterators,
errors, and testable CLI design. Mature crates may decode established formats;
the original work is the analysis above that layer.

| Project | What it does | Main practice | Status |
| --- | --- | --- | --- |
| [`wav-sheriff`](rust/wav-sheriff/) | Locates clipping, DC offset, flatlines, dropouts, and abrupt discontinuities in PCM WAVE recordings. | Streaming statistics, per-channel state, numeric conversions, finding coalescence, error handling | Planned |
| [`tracklint`](rust/tracklint/) | Audits GPX tracks for teleports, reversed timestamps, duplicates, gaps, and local speed or elevation spikes. | `Option`, nested iterators, time arithmetic, sliding windows, geodesic math, domain enums | Planned |
| [`grooveprint`](rust/grooveprint/) | Produces a rhythmic fingerprint of a MIDI file and reports overlapping, unmatched, or hanging notes. | Event enums, pattern matching, maps and queues, absolute timelines, tempo maps, integer timing | Planned |

#### Rust MVP Boundaries

**`wav-sheriff`**

- Read PCM integer WAVE files with [`hound`](https://docs.rs/hound/latest/hound/).
- Report evidence and exact time ranges; do not repair or master audio.
- Start with 16-bit and 24-bit samples and transparent, configurable thresholds.
- Test with synthetic clipping, silence, DC bias, and discontinuities.

**`tracklint`**

- Read GPX 1.0/1.1 tracks with the [`gpx`](https://docs.rs/gpx/latest/gpx/) crate.
- Never calculate across a track-segment boundary.
- Rank suspicious points and explain each finding; do not silently delete data.
- Ignore routes, standalone waypoints, and vendor extensions initially.

**`grooveprint`**

- Read Standard MIDI Files with [`midly`](https://docs.rs/midly/latest/midly/).
- Support ticks-per-quarter-note timing first and reject SMPTE timing clearly.
- Analyze onset phase, swing, accents, durations, polyphony, and note integrity.
- Do not add playback, synthesis, live MIDI, editing, or a custom MIDI decoder.

### C

C projects work directly with small, bounded formats and memory layouts. They
emphasize byte-level I/O, explicit allocation, defensive parsing, and simple
algorithms rather than recreating the Rust projects with fewer guardrails.

| Project | What it does | Main practice | Status |
| --- | --- | --- | --- |
| [`blob-map`](c/blob-map/) | Finds connected regions in PGM grayscale images and emits labels, area, bounds, and centroids. | Mixed text/binary parsing, 2D indexing, queues, dynamic arrays, image serialization | Planned |
| [`tar-suspicion`](c/tar-suspicion/) | Audits USTAR archives for bad checksums, duplicate paths, traversal paths, unsafe links, and inconsistent sizes. | Fixed-block I/O, octal parsing, checksums, path normalization, overflow checks | Planned |
| [`pcap-chatter`](c/pcap-chatter/) | Summarizes bidirectional TCP and UDP conversations from classic PCAP captures. | Endianness, layered binary parsing, bounds checks, hash tables, timestamps, sorting | Planned |

#### C MVP Boundaries

**`blob-map`**

- Read one 8-bit P2 or P5 PGM image without an image library.
- Apply a caller-provided threshold and label 4-connected components.
- Print component statistics and write a labeled PGM result.
- Do not add JPEG/PNG support, computer-vision dependencies, or a GUI.

**`tar-suspicion`**

- Read POSIX USTAR in 512-byte blocks and never extract archive contents.
- Validate checksums, padding, sizes, termination, duplicate names, and paths.
- Flag absolute paths, `..` traversal, and suspicious link targets.
- Reject unsupported GNU/PAX extensions clearly instead of half-supporting them.

**`pcap-chatter`**

- Read classic PCAP offline; reject PCAPNG and non-Ethernet captures clearly.
- Decode Ethernet, IPv4, TCP, and UDP one layer at a time from bytes.
- Aggregate packet counts, byte counts, duration, ports, and TCP flags.
- Report fragments and truncation; do not reassemble traffic or capture live data.

## Structure

```text
.
├── AGENTS.md
├── README.md
├── c/
│   └── <project>/
└── rust/
    └── <project>/
```

Each project owns its source, tests, fixtures, build files, and a short README.
There is no root workspace or shared library until repeated code creates a real
need for one.

## Research Notes

The shortlist was selected after comparing current project catalogs with
primary format specifications and ecosystem documentation. Common suggestions
such as shells, compilers, databases, HTTP servers, command clones, and CRUD
apps were discarded. Rust and C were then given different kinds of work rather
than duplicate implementations.

Sources that directly shaped the scopes:

- [Command Line Applications in Rust: Testing](https://rust-cli.github.io/book/tutorial/testing.html) recommends separating testable domain logic from CLI I/O and combining unit and integration tests.
- [`midly` documentation](https://docs.rs/midly/latest/midly/) confirms that MIDI decoding is a mature crate-level concern, leaving event analysis as the project work.
- [GPX 1.1 schema](https://www.topografix.com/GPX/1/1/) defines optional point data and track segments as separate continuous spans, which sets `tracklint`'s correctness rules.
- [Library of Congress WAVE description](https://www.loc.gov/preservation/digital/formats/fdd/fdd000001.shtml) documents WAVE as a RIFF wrapper that can contain several encodings, motivating the explicit PCM-only boundary.
- [PCAP Capture File Format](https://datatracker.ietf.org/doc/html/draft-ietf-opsawg-pcap) defines the compact classic format, endian variants, record lengths, and hostile-input validation requirements.
- [GNU tar: Basic Tar Format](https://www.gnu.org/software/tar/manual/html_node/Standard.html) documents USTAR's 512-byte blocks, octal fields, checksum calculation, and entry types.
- [Netpbm PGM specification](https://netpbm.sourceforge.net/doc/pgm.html) describes a deliberately simple grayscale format suitable for implementing image algorithms without a format library.
- [Build Your Own X](https://github.com/codecrafters-io/build-your-own-x), [Project Based Learning](https://github.com/practical-tutorials/project-based-learning), and [Coding Challenges](https://codingchallenges.fyi/challenges/intro) were used as negative filters for common or oversized project families.

Research reviewed on **2026-07-21**. Specifications remain the source of truth
when implementation begins.

## Ground Rules

- Keep at most three active or planned projects per language at this level.
- Finish the documented MVP before adding stretch goals.
- Prefer the standard library; use a dependency when it avoids rebuilding a
  mature parser or other non-learning objective.
- Treat every external file as malformed until validated.
- Include focused unit tests and at least one end-to-end test per project.
- Raise the difficulty or add Go only when the repository owner explicitly
  decides to do so.
