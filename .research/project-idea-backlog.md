# Researched Project Idea Backlog

This file preserves promising ideas that did not make the current project
catalog. They are **not active or planned projects**, and therefore do not count
toward the three-project-per-language limit in `README.md`.

An idea should move into the catalog only when the repository owner selects it,
chooses which existing catalog entry it replaces, and confirms that the current
difficulty level is appropriate. Recheck every linked specification and library
before implementation; this research was performed on **2026-07-21**.

## Selection Method

The research compared current project collections, format specifications,
ecosystem documentation, and existing implementations. Ideas were favored when
they combined several fundamentals, produced something useful, had a defensible
MVP boundary, and fit one language naturally.

The following common families were deliberately filtered out:

- Compilers, interpreters, shells, databases, web servers, and operating systems
- CRUD applications, to-do lists, calculators, and single-concept exercises
- Straight clones of commands such as `cat`, `grep`, `wc`, `sort`, and `tar`
- Large protocol implementations whose edge cases would consume the project
- Projects requiring a GUI or application framework before domain work begins

Broad catalogs used as negative filters:

- [Build Your Own X](https://github.com/codecrafters-io/build-your-own-x)
- [Project Based Learning](https://github.com/practical-tutorials/project-based-learning)
- [Coding Challenges](https://codingchallenges.fyi/challenges/intro)
- [GeeksforGeeks C Projects](https://www.geeksforgeeks.org/c/c-projects/)
- [CodeCrafters Rust project survey](https://codecrafters.io/blog/rust-projects)

## Deferred Rust Ideas

These ideas suit Rust's enums, iterators, explicit error types, safe byte
handling, and ability to place original analysis above a mature parser crate.

### Offline DNS Transaction Profiler

Read a packet capture and pair DNS requests with responses to report latency,
response codes, retries, unanswered queries, record types, and unusually large
answers.

**Bounded MVP**

- Accept classic PCAP containing Ethernet, IPv4, UDP, and DNS only.
- Match transactions by endpoints and DNS transaction ID.
- Decode header flags, questions, and common answer records.
- Detect compression-pointer loops and truncated records safely.
- Report unsupported IPv6, TCP, fragments, and link types without decoding them.

**Concepts**

- Nested byte-slice parsing and endianness
- Bitfields, enums, and checked offsets
- Hash maps, transaction state, and timestamps
- DNS name compression and cycle detection
- Errors that preserve protocol-layer context

**Why deferred**

Classic PCAP, Ethernet, IPv4, UDP, and DNS create five parsing layers before the
analysis begins. DNS compression also makes malformed-input handling unusually
important. This is better after completing a smaller binary-format project.

**Research**

- [PCAP Capture File Format](https://datatracker.ietf.org/doc/html/draft-ietf-opsawg-pcap)
- [RFC 1035: Domain Names - Implementation and Specification](https://www.rfc-editor.org/rfc/rfc1035.html)
- [`pcap-parser`](https://github.com/rusticata/pcap-parser)

### PNG Metadata Exposure-Budget Inspector

Inventory metadata in PNG and APNG files, estimate how much non-image data they
carry, and flag information that may be private or unexpectedly large.

**Bounded MVP**

- Walk PNG chunks and validate their declared lengths and CRCs.
- Inventory text, EXIF, color-profile, timestamp, and physical-scale chunks.
- Detect trailing bytes and unknown critical chunks.
- Apply a documented policy that classifies findings without modifying files.
- Print a concise privacy and size report.

**Concepts**

- Streaming binary I/O and big-endian fields
- CRC calculation and checked arithmetic
- Chunk enums and policy-driven findings
- Text decoding and bounded allocation
- Distinguishing format validity from privacy advice

**Why deferred**

The actively maintained `pngcheck` already validates PNG/APNG and reports much
of its metadata. A privacy-policy layer is distinct, but the core project would
still resemble an existing utility more than the selected Rust analyzers do.

**Research**

- [PNG Specification, Third Edition](https://www.w3.org/TR/png-3/)
- [`pngcheck`](https://github.com/pnggroup/pngcheck)

### Calendar Transition-Load Auditor

Analyze an iCalendar schedule for overlaps, back-to-back meetings, fragmented
focus time, short breaks, and days with excessive context switching.

**Bounded MVP**

- Read a single `.ics` calendar through a mature iCalendar crate.
- Analyze events inside a caller-provided date range.
- Merge overlaps and classify free intervals by duration.
- Report daily meeting load, transition count, and uninterrupted focus blocks.
- Require expanded events initially rather than implementing recurrence rules.

**Concepts**

- Date/time types and timezone-aware comparisons
- Interval sorting, merging, and gap analysis
- Recurrence-related input constraints
- Domain findings and configurable policies
- Human-readable reporting

**Why deferred**

iCalendar looks simple but includes folded lines, floating times, timezone
components, recurrences, exclusions, and all-day events. Excluding recurrence
makes the MVP less useful, while supporting it correctly raises the difficulty
well beyond the current level.

**Research**

- [RFC 5545: Internet Calendaring and Scheduling Core Object Specification](https://www.rfc-editor.org/rfc/rfc5545.html)
- [`icalendar-rs`](https://github.com/hoodie/icalendar)

### Mbox Conversation Archaeology Report

Reconstruct email threads from an mbox archive and summarize missing parents,
participants, thread depth, response latency, and conversations that died out.

**Bounded MVP**

- Read one documented mbox variant and parse messages with a mail crate.
- Build threads from `Message-ID`, `In-Reply-To`, and `References` headers.
- Report roots, missing ancestors, branching, depth, and response-time ranges.
- Ignore HTML rendering and attachments beyond counting them.
- Never transmit, rewrite, or index mail outside the current run.

**Concepts**

- Streaming records and borrowed message data
- Maps, trees, missing references, and cycle protection
- MIME headers, optional fields, and date parsing
- Privacy-aware fixtures and reporting
- Separating parser errors from incomplete real-world data

**Why deferred**

RFC 4155 documents that mbox has no single authoritative historical format.
Separator escaping, malformed mail, missing IDs, unusual dates, and MIME
variations would make data cleanup dominate an otherwise good graph project.

**Research**

- [RFC 4155: The application/mbox Media Type](https://www.rfc-editor.org/rfc/rfc4155.html)
- [`mailparse`](https://docs.rs/mailparse/latest/mailparse/)

## Deferred C Ideas

These ideas suit C's direct byte access, explicit memory management, POSIX APIs,
and small dependency-free tools. They remain separate from similar Rust ideas
so the language tracks do not become ports of each other.

### RIFF/WAVE Auditor and Waveform Renderer

Validate a deliberately small PCM WAVE subset, compute channel statistics, and
render a dependency-free waveform as a PGM image.

**Bounded MVP**

- Walk RIFF chunks instead of assuming a fixed 44-byte header.
- Support mono or stereo 8-bit and 16-bit PCM only.
- Validate sizes, padding, byte rate, block alignment, and sample divisibility.
- Report minimum, maximum, RMS, DC offset, and clipped samples per channel.
- Downsample sample ranges into a fixed-size raw PGM waveform.

**Concepts**

- Little-endian chunk parsing and odd-byte padding
- Signed and unsigned sample representation
- Streaming statistics and overflow-aware arithmetic
- Per-channel state and output serialization
- Binary test-fixture construction

**Why deferred**

It is a strong C project, but `wav-sheriff` currently reserves audio analysis
for Rust. Keeping both would make the language tracks overlap. This candidate
could replace that Rust project later if direct format work becomes preferable.

**Research**

- [McGill WAVE File Specifications](https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html)
- [Library of Congress WAVE description](https://www.loc.gov/preservation/digital/formats/fdd/fdd000001.shtml)
- [`dr_wav.h`](https://github.com/mackron/dr_libs/blob/master/dr_wav.h)
- [Netpbm PGM specification](https://netpbm.sourceforge.net/doc/pgm.html)

### Standard MIDI Timeline and Note-State Linter

Parse a constrained Standard MIDI File and report stuck notes, unmatched
note-offs, invalid events, missing end markers, and peak polyphony.

**Bounded MVP**

- Parse format 0 `MThd` and `MTrk` chunks.
- Decode four-byte-limited variable-length quantities and running status.
- Handle note, control, program, and common meta events.
- Treat note-on with velocity zero as note-off.
- Produce a tick-based CSV timeline and explicit structural findings.
- Reject format 2, playback, synthesis, editing, and system-exclusive decoding.

**Concepts**

- Big-endian binary I/O and variable-length integers
- Stateful event decoding and running status
- Dynamic arrays and active-note tables
- Pairing note lifetimes and tracking polyphony
- Defensive length and data-byte validation

**Why deferred**

The parser and state machine are feasible but form the most difficult C idea in
the research set. It also overlaps the selected Rust `grooveprint` domain. It is
best retained as a possible future replacement, not a parallel implementation.

**Research**

- [Standard MIDI-File Format Spec 1.1](https://midimusic.github.io/tech/midispec.html)
- [Stanford Standard MIDI File Structure](https://ccrma.stanford.edu/~craig/14q/midifile/MidiFileFormat.html)
- [The MIDI File Format](https://philjonas.github.io/c-midi-writer/midi_file_format.html)

### Raw DNS Packet Microscope

Send one DNS request over UDP and display the response structure, decoded
records, TTLs, flags, and compression-pointer paths.

**Bounded MVP**

- Construct one query for a caller-provided name and record type.
- Send it to an explicitly selected resolver with a timeout.
- Decode questions and A, AAAA, MX, NS, TXT, and CNAME answers.
- Bound every label, pointer, section count, and output allocation.
- Report truncated responses instead of adding DNS-over-TCP initially.

**Concepts**

- POSIX UDP sockets, timeouts, and network byte order
- Bitfields and variable-length records
- Safe offset arithmetic in C
- Compression pointers and loop detection
- Separating transport failures from malformed responses

**Why deferred**

DNS clients and resolvers are increasingly common challenge projects. Name
compression also introduces a sharp jump in parser risk. The offline PCAP
summarizer offers networking concepts without depending on live network state.

**Research**

- [RFC 1035: Domain Names - Implementation and Specification](https://www.rfc-editor.org/rfc/rfc1035.html)
- [Build Your Own DNS Resolver](https://codingchallenges.fyi/challenges/challenge-dns-resolver/)
- [`raw-dns-client`](https://github.com/adrianochi/raw-dns-client)

### Linux Process-Churn Flight Recorder

Sample `/proc` periodically and record process starts, exits, CPU deltas, and
I/O deltas as events rather than building another interactive `top` clone.

**Bounded MVP**

- Snapshot numeric `/proc/<pid>` directories at a fixed interval.
- Parse identity, start time, CPU counters, and I/O counters.
- Distinguish PID reuse by combining PID with process start time.
- Emit CSV rows for starts, exits, and unusually large interval changes.
- Handle disappearing processes and permission errors without aborting a scan.

**Concepts**

- Directory traversal and robust text parsing
- Monotonic clocks, sampling, and counter deltas
- Hash tables, process identity, and PID reuse
- Signal handling and clean shutdown
- Races caused by processes disappearing during reads

**Why deferred**

It is Linux-only, while the repository currently develops on macOS. It is also
close to the heavily represented `top` project family, despite the event-log
framing making it more distinctive.

**Research**

- [`proc_pid_stat(5)`](https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html)
- [`proc_pid_io(5)`](https://man7.org/linux/man-pages/man5/proc_pid_io.5.html)
- [Build Your Own `top`](https://codingchallenges.fyi/challenges/challenge-top/)
- [`procps-ng`](https://gitlab.com/procps-ng/procps)

## Ideas Rejected Earlier

These were considered but not retained as full backlog candidates:

- A generic PNG validator: too close to `pngcheck` without the privacy angle.
- A tar extractor: common, security-sensitive, and less distinctive than a
  read-only safety auditor.
- A live packet sniffer: requires privileges and platform capture APIs before
  the interesting analysis starts.
- A universal WAVE reader: real implementations demonstrate that broad codec,
  metadata, RF64, and extensible-format support is far beyond mini-project size.
- A route planner or GPX map viewer: UI and mapping concerns obscure the data
  consistency analysis selected for `tracklint`.
- MIDI playback or synthesis: hardware and real-time audio concerns obscure the
  event-state analysis selected for `grooveprint`.

## Promotion Checklist

Before promoting an idea into `README.md`:

- Confirm which current project it replaces; do not exceed three per language.
- Recheck specifications and the latest crate or tool behavior online.
- Write an explicit MVP and a list of non-goals.
- Ensure it does not duplicate a project in the other language.
- Confirm it still matches the owner's chosen difficulty level.
- Choose tiny, legal, non-sensitive fixtures before implementation begins.
