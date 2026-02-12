# css/css-viewport/zoom/reference/text-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/text-shadow-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
}
div {
  font-size: calc(1rem * var(--scale));
  text-shadow: calc(5px * var(--scale)) calc(5px * var(--scale)) hotpink;
}
.zoom {
  --scale: 2;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
