# WPT `<style>` false positives

This document tracks Web Platform Tests (WPT) `<style>` blocks where `css_inspector` reports
**errors**, but the stylesheet content should be considered **valid** in context (i.e. as it’s
actually consumed by a browser).

Each entry is keyed by the WPT-style id: `<wpt-path>#style[N]`.

## Fixed

### `css/css-content/quotes-030.html#style[0]` / `css/css-content/reference/quotes-030-ref.html#style[0]` / `css/css-content/quotes-033.html#style[0]`

- Issue: `quotes: auto` was treated as invalid.
- Fix: accept `auto` for `quotes` in the suite-driven value validator.

```css
body { quotes: auto; }
.inner { quotes: auto; }
```

### `css/css-values/attr-crash.html#style[0]`

- Issue: `<ident>` type annotations in `attr(... type(<ident>))` triggered `Invalid input.` due to
  the top-level `<` “HTML in CSS” guard.
- Fix: treat `<ident>` as an allowed CSS type annotation.

```css
#div {
  --prop: attr(data-foo type(<ident>));
}
```

### `css/css-cascade/scope-nesting.html#style[4]`

- Issue: WPT contains the literal string `<stylesheet>` inside a `<script>` block. The WPT
  `<style>` extractor was substring-based and incorrectly treated this as an opening `<style>` tag,
  producing bogus “style blocks”.
- Fix: make `<style>` extraction skip `<script>...</script>` contents, and ignore `<style`-looking
  text inside HTML comments.

```css
@scope (#div) {
  :scope {
    z-index: 1;
    & {
      z-index: 2;
    }
  }
}
```

### `css/css-flexbox/flexbox-align-self-vert-003-ref.xhtml#style[0]` (and `*-rtl-003-ref.xhtml`, `*-004-ref.xhtml`, `*-rtl-004-ref.xhtml`)

- Issue: these are XHTML reference files containing XML comments inside `<style>`. In XML, comment
  nodes are not part of the stylesheet text, but our extractor slices raw bytes between `<style>`
  and `</style>`, so it includes `<!-- ... -->` and triggers `Unbalanced braces.` (via an
  unterminated string from an apostrophe inside the comment).
- Fix: when extracting style blocks from XML-ish documents (`.xht`, `.xhtml`, `.xml`, `.svg`), strip
  `<!-- ... -->` comment markup from the extracted stylesheet text before validation (unless the
  stylesheet appears to be CDATA-wrapped).

```css
.flex-end {
  background: orange;
  float: right;
}
<!-- XML comment in XHTML; not stylesheet text -->
.centerParent {
  text-align: center;
}
```

### `css/css-speech/voice-family-integer.html#style[0]`

- Issue: `voice-family: female 1` was treated as invalid.
- Fix: accept integer indexes in `voice-family`.

```css
p.voice1 {voice-family: female 1;}
p.voice2 {voice-family: female 2;}
```

### `css/CSS2/syntax/characters-0080-009F-001.xht#style[0]`

- Issue: identifiers containing non-ASCII characters (U+0080+) were rejected for counter names
  (`counter-increment`).
- Fix: validate identifier-like tokens using the shared identifier parser (allowing non-ASCII and
  escapes).

```css
div p { counter-increment: \81\82\83\9d\9e\9f }
```

### `css/css-images/cross-fade-legacy-2-crash.html#style[0]`

- Issue: legacy `-webkit-cross-fade(...)` was treated as invalid for `<image>` properties.
- Fix: accept `-webkit-cross-fade(...)` as a background-image-like function.

```css
.test { background-image: -webkit-cross-fade(none, none, calc(13% + 1% * sign(1em - 1px))); }
```

## Open
