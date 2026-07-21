# Tar Suspicion

Tar Suspicion is a read-only safety auditor for POSIX USTAR archives. It answers
whether an archive appears structurally consistent and whether its entries
contain paths or links that would be dangerous to trust during extraction.

Its report highlights invalid checksums, inconsistent sizes, malformed archive
termination, duplicate paths, absolute paths, parent-directory traversal, and
suspicious link targets.

The project never extracts or modifies archive contents. It is not a replacement
for `tar` or a universal validator for every tar extension.
