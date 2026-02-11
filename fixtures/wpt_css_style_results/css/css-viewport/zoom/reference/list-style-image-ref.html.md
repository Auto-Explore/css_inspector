# css/css-viewport/zoom/reference/list-style-image-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/list-style-image-ref.html"
}
```

## style[0]

```css

body {
  list-style-image: url('#marker');
  --scale: 1;
}
ul {
  font-size: calc(1rem * var(--scale));
  margin-block-start: calc(1rem * var(--scale));
  margin-block-end: calc(1rem * var(--scale));
  margin-inline-start: 0px;
  margin-inline-end: 0px;
  padding-inline-start: calc(40px * var(--scale));
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
