# css/css-sizing/range-percent-intrinsic-size-2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/range-percent-intrinsic-size-2-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

input { margin: 2px; }

input.b {
  min-height: 0;
  background: lime;
}

input.mb {
  min-height: 0;
  max-height: 100%;
  background: lime;
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
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
