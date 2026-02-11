# css/css-viewport/zoom/reference/iframe-zoom-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/iframe-zoom-ref.html"
}
```

## style[0]

```css

body {
  --iframe-width: 128px;
  --iframe-height: 64px;
  --scale: 1;
}
iframe {
  border: none;
  width: calc(var(--iframe-width) * var(--scale));
  height: calc(var(--iframe-height) * var(--scale));
}
.scale {
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
