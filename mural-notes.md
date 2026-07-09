# Mural Notes

## Goal

Take an input image, break it into 20x20 pixel chunks, and replace each
chunk with a single best-matching color from a fixed palette of 50
colors, such that the resulting "mural" still resembles the original
image as closely as possible.

## Step 1: Summarize the 20x20 chunk into one representative color

- **Mean (average) RGB** — simplest, fast, good default. Works well for
  smooth/gradient chunks.
- **Median color** — more robust if a chunk has outliers (e.g., a small
  bright highlight or noise) that would skew a mean.
- **Dominant color (mode via histogram/k=1 clustering)** — best when a
  chunk has two contrasting regions (e.g., a sharp edge between object
  and background); the average would produce a muddy color that exists
  nowhere in the chunk, while dominant color picks what's actually most
  present.

For a first version, average is fine; dominant color is a nice upgrade
if edges look muddy.

## Step 2: Do this in the right color space

Averaging/matching in plain sRGB is what most naive implementations do,
but sRGB is **not perceptually uniform** — equal numeric distances don't
correspond to equal perceived differences, and averaging in
gamma-encoded space slightly darkens perceived brightness.

Since perceptual accuracy is the priority here:

1. Convert both the image chunk and the 50-color palette to **CIELAB**
   (Lab) once, up front.
2. Average the chunk's pixels in Lab (or convert to linear RGB, average,
   then convert to Lab — either is reasonable; averaging directly in Lab
   is simpler and looks good in practice).
3. Compare the chunk's Lab value against the 50 palette Lab values to
   find the closest one.

## Step 3: Distance/matching metric

- **Euclidean distance in Lab** — big improvement over RGB, simple to
  implement (`sqrt(ΔL² + Δa² + Δb²)`), and good enough for most palette-
  matching use cases.
- **CIEDE2000** — the gold-standard perceptual difference formula, more
  complex (weighting terms, hue rotation) but more accurate especially
  for hues humans are sensitive to. Worth it since there are only 50
  palette colors and the goal is the *best* one, not just a decent one.
- With only 50 palette colors, brute-force comparison per chunk (50
  distance checks) is trivial performance-wise — no need for k-d trees
  or other acceleration structures.

**Recommendation:** convert palette + per-chunk average to Lab, use
CIEDE2000 (or start with Lab Euclidean distance for simpler math first,
upgrade later).

## Dithering (optional, for reducing banding)

With just 50 colors, large flat areas of subtle gradients will look
"banded" — noticeable jumps between blocks. Dithering fixes this by not
treating each block in isolation:

- **How it works**: after choosing the nearest palette color for a
  chunk, compute the *error* (original average color minus chosen
  palette color). Instead of discarding it, distribute a fraction of
  that error into neighboring chunks' target colors before they get
  matched (classic Floyd–Steinberg diffusion, just done at block
  granularity instead of pixel granularity).
- **Pros**: smoother-looking gradients, less visible banding, image
  reads better from a distance (which matters for a "mural").
- **Cons**: adds implementation complexity and ordering dependency (must
  process chunks in raster order, left-to-right/top-to-bottom, since
  each chunk's target depends on accumulated error from earlier ones).
  Also slightly reduces local color accuracy of individual chunks in
  exchange for global smoothness.

## Overall recommendation / thought process

Implement the straightforward independent nearest-color match first:

1. Mean (or dominant) color per chunk.
2. Convert to Lab.
3. Nearest palette color via CIEDE2000 (or Lab Euclidean distance as a
   simpler starting point).

See how the mural looks before adding block-level error diffusion.
Murals viewed at a distance with fairly large 20x20 blocks often look
fine without dithering, since each block is already fairly coarse —
only add dithering afterward if banding turns out to be a visible
problem in practice.
