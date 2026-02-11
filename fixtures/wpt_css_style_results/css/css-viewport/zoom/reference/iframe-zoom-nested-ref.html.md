# css/css-viewport/zoom/reference/iframe-zoom-nested-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/iframe-zoom-nested-ref.html"
}
```

## style[0]

```css

body {
  columns: 2;
  --iframe-width: 256px;
  --iframe-height: 128px;
  --scale: 1;
}
iframe {
  border: none;
  width: calc(var(--iframe-width) * var(--scale));
  height: calc(var(--iframe-height) * var(--scale));
}
.scale {
  --scale: 1.5;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
