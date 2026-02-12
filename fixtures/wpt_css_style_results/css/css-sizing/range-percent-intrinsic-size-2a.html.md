# css/css-sizing/range-percent-intrinsic-size-2a.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/range-percent-intrinsic-size-2a.html"
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

div {
  display: inline-block;
  border:1px solid;
}

.grid {
  display: inline-grid;
  grid: auto / min-content;
  place-items: start;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
