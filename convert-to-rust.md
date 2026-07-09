# Notes: Converting `quad.py` to Rust

## The `error` function

```python
def error(total, squared, n):
    return (squared - (total * total / n)) / n
```

Computes the **variance of pixel intensities** in a quadrant, using the
sum-of-squares identity:

```
variance = E[x^2] - (E[x])^2 = squared/n - (total/n)^2
```

Rearranged algebraically as `(squared - total^2/n) / n`, so it can be
computed directly from precomputed sums (via integral images) instead of
iterating over every pixel.

- `total`   = sum of grayscale pixel values in the region
- `squared` = sum of grayscale pixel values squared in the region
- `n`       = number of pixels in the region

### How it's used

For each of the four child quadrants (top-left, top-right, bottom-left,
bottom-right), `error` is computed right after the quadrant sums are
derived from the integral images. The result (negated, since Python's
`heapq` is a min-heap) is pushed onto a priority queue along with the
quadrant's position and size.

- **Higher variance** = more visual detail/contrast in that region =
  higher priority to be split again.
- **Lower variance** = flat/uniform region = left alone.

In Rust, `std::collections::BinaryHeap` is a max-heap by default, so the
negation trick used in Python is not needed — just push `error` directly.

## What the whole script does

This is a **quadtree image/video segmentation tool**.

### 1. Setup (`quad` function)

For an image, it builds three *integral images* (summed-area tables):

- One for the original color image (used to compute average color).
- One for grayscale values (used in the variance formula).
- One for grayscale values squared (used in the variance formula).

Integral images allow computing the sum over any rectangular region in
O(1) time instead of O(pixels), by padding with an edge row/column and
taking cumulative sums along both axes.

### 2. Main loop

Starting with the whole image as one quadrant:

1. Split the current quadrant into 4 sub-quadrants (TL, TR, BL, BR).
2. For each sub-quadrant:
   - Compute its average color using the color integral image.
   - Paint that flat average color into the output image (`edited`).
   - Optionally draw a black border around it.
   - Compute its variance via `error` (using grayscale integral images).
   - Push it onto a max-priority-heap keyed by variance.
3. Pop the quadrant with the **highest variance** (most "detail") off the
   heap — this becomes the next quadrant to subdivide.
4. Repeat for `iterations` steps, or stop early if quadrants become
   smaller than `min_width`/`min_height`, or the heap is empty.

**Net effect:** Areas with lots of detail/contrast get recursively
subdivided into smaller blocks, while flat/uniform areas stay as large
blocks of solid average color — producing the classic "quadtree mosaic"
effect, concentrating detail where the image actually has it.

### 3. CLI / IO layer

- `parse_args` — parses command-line args: input/output paths, iteration
  count, quality, borders, audio inclusion, minimum block width/height.
- `main` — tries to load input as a still image; if that fails, assumes
  it's a video.
- `quadtree_image` — runs the quadtree algorithm once on a still image
  and saves the result.
- `quadtree_video` — runs the quadtree algorithm independently per frame
  of a video (reusing the same quadrant list/buffer across frames, but
  clearing it each frame) and writes an output video via `ffmpeg`,
  optionally muxing in the original audio.

## Deep dive: the sum-of-squares / variance trick, with C++ pseudo-code

### The concept

`quad.py`'s `error` function measures how much *detail/variation* is in a
region by computing its **variance** — how spread out the pixel
intensities are from their mean. A flat, uniform patch has variance ≈ 0;
a noisy/high-contrast patch has high variance.

The naive way to compute variance for a chunk is two passes:
1. Compute the mean.
2. Compute the average squared deviation from that mean.

But there's an algebraic trick that lets you do it in **one pass**,
using this identity:

```
variance = E[x^2] - (E[x])^2
```

In words: *"the average of the squares"* minus *"the square of the
average"*. This is mathematically equivalent to the classic two-pass
definition, but you only need to accumulate two running sums as you
scan pixels once: `sum(x)` and `sum(x^2)`.

That's exactly what `error(total, squared, n)` does:
```python
return (squared - (total * total / n)) / n
```
where `total = sum(x)`, `squared = sum(x^2)`, `n` = pixel count.
Rearranged: `squared/n - (total/n)^2` = `E[x^2] - (E[x])^2`.

### C++ pseudo-code for a 20x20 chunk

```cpp
// Direct one-pass computation for a single 20x20 chunk.
double computeVariance(const uint8_t chunk[20][20]) {
    double total = 0.0;   // sum(x)
    double squared = 0.0; // sum(x^2)
    int n = 20 * 20;      // 400 pixels

    for (int y = 0; y < 20; ++y) {
        for (int x = 0; x < 20; ++x) {
            double v = chunk[y][x]; // grayscale intensity
            total += v;
            squared += v * v;
        }
    }

    // variance = E[x^2] - (E[x])^2, rearranged to avoid a second pass
    return (squared - (total * total / n)) / n;
}
```

### Why bother with this identity instead of two passes?

For a single isolated 20x20 chunk, it barely matters — 400 pixels is
nothing. The real payoff shows up when you need variance for **many
overlapping/nested regions repeatedly**, like `quad.py` does across a
quadtree of shrinking rectangles.

The trick: if you precompute a **running sum image** (integral image) of
`x` and of `x^2` over the *entire* image once, then the sum over *any*
rectangular region — regardless of size or position — can be looked up
in O(1) using 4 array reads (inclusion-exclusion), instead of
re-scanning every pixel in that region every time.

```cpp
// Conceptual: integral images built once for the whole image.
// I[y][x]  = sum of all pixels in rectangle (0,0)-(x,y)
// Isq[y][x] = sum of all squared pixels in rectangle (0,0)-(x,y)

double rectSum(const double I[/*H+1*/][/*W+1*/], int x0, int y0, int x1, int y1) {
    // Sum over rectangle [x0,x1) x [y0,y1), using padded integral image.
    return I[y1][x1] - I[y0][x1] - I[y1][x0] + I[y0][x0];
}

double regionVariance(const double I[][/*W+1*/], const double Isq[][/*W+1*/],
                       int x0, int y0, int x1, int y1) {
    int n = (x1 - x0) * (y1 - y0);
    double total = rectSum(I, x0, y0, x1, y1);
    double squared = rectSum(Isq, x0, y0, x1, y1);
    return (squared - (total * total / n)) / n; // same formula as before
}
```

So: for a **one-off 20x20 chunk**, just loop and accumulate
`total`/`squared` directly (first snippet). For **many chunks over the
same image** (like quadtree recursion, or a mural's grid of chunks),
build the two integral images once, then get each chunk's variance in
O(1) via `regionVariance` — no re-scanning pixels per chunk.

### Does it need to be grayscale intensity?

No — grayscale is just a design choice `quad.py` made, not a
requirement of the math. The sum-of-squares/variance trick works on
**any single scalar value per pixel**. Alternatives, and their
tradeoffs:

1. **Grayscale intensity (what `quad.py` does)** — convert each pixel to
   a single luminance value first (e.g.
   `0.299R + 0.587G + 0.114B`), then compute variance on that scalar.
   - *Pro:* Simple — one running sum/sum-of-squares pair, one integral
     image pair, one variance number to compare across regions.
   - *Con:* Blind to pure color/chrominance changes with the same
     brightness. Two colors that are equally bright but very different
     hues (e.g. flat red vs. flat green of the same luminance) would
     look like *zero variance* even though visually they're totally
     different regions.

2. **Per-channel variance, combined** — compute variance separately for
   R, G, and B (3 separate `total`/`squared` sums, 3 integral image
   pairs), then combine, e.g. sum them or take the max:
   ```cpp
   double varR = regionVariance(IR, IRsq, ...);
   double varG = regionVariance(IG, IGsq, ...);
   double varB = regionVariance(IB, IBsq, ...);
   double colorError = varR + varG + varB; // or max(varR, varG, varB)
   ```
   - *Pro:* Captures color variation, not just brightness variation.
   - *Con:* 3x the integral images/memory/computation, and "sum of
     per-channel variances" isn't a perceptually meaningful unit by
     itself (just a heuristic).

3. **Perceptual color space (Lab) variance** — convert to Lab first,
   then compute variance on L, a, b (or on a combined distance from the
   mean Lab color). This captures both brightness and chrominance
   detail, and Lab differences correlate much better with what humans
   perceive as "detail."

**Practical takeaway:** `quad.py` uses grayscale purely for **simplicity
and speed** — one scalar, one set of integral images, and it's a
reasonable proxy for "visual detail" since most edges/structure show up
in luminance anyway. If a Rust port cares more about color fidelity
(e.g. a region that's flat luminance but split between two saturated
colors), computing variance per-channel or in Lab would catch cases
pure grayscale variance misses — at the cost of extra integral images
and slightly more computation.

## Key pieces to replicate in Rust

1. **Integral image construction** — cumulative sums along rows then
   columns, with one row/column of edge-padding.
2. **O(1) rectangle-sum queries** — using the standard 4-point integral
   image lookup (inclusion-exclusion on the padded table).
3. **The variance formula** in `error`.
4. **A max-heap** for picking the next quadrant to split — `BinaryHeap`
   in Rust is a max-heap natively, so no negation trick is needed (just
   be careful with `f64`/`f32` ordering, since floats don't implement
   `Ord` — you'll likely need a wrapper type or `OrderedFloat` from the
   `ordered-float` crate).
5. **Image/video IO** — Python uses `imageio`/`imageio_ffmpeg`; in Rust,
   consider crates like `image` for stills and `ffmpeg-next` or shelling
   out to the `ffmpeg` binary for video frame decode/encode.
