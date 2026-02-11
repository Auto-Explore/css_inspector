# css/css-sizing/range-percent-intrinsic-size-2a-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/range-percent-intrinsic-size-2a-ref.html"
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
