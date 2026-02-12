# css/css-sizing/range-percent-intrinsic-size-2.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/range-percent-intrinsic-size-2.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

input { margin: 2px; }

input.b {
  height: 50%;
  min-height: min-content;
  background: lime;
}

input.mb {
  max-height: 50%;
  min-height: min-content;
  background: lime;
}

input.b.min-auto, input.mb.min-auto {
  min-height: auto;
}

.n {
  -webkit-appearance: none;
}

div {
  display: inline-block;
  border:1px solid;
}

.grid {
  display: inline-grid;
  grid: auto / min-content;
  place-items: start;
}
input[orient="vertical"] {
  -webkit-appearance: slider-vertical;
  -webkit-appearance: range;
  appearance: auto;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
