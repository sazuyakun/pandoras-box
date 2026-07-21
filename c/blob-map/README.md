# Blob Map

Blob Map is a small image-analysis tool for finding connected regions in PGM
grayscale images. A chosen brightness threshold separates the image into
foreground and background, allowing distinct shapes or clusters to be treated
as individual components.

For each component, the tool reports useful properties such as area, bounding
box, and center point. It also produces a labeled image that makes the detected
regions visually distinguishable.

The project is not a general photo editor, object-recognition system, or
full-format computer-vision suite.
