# css/css-viewport/zoom/border-width-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/border-width-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
}
.box {
  width: calc(60px * var(--scale));
  height: calc(60px * var(--scale));
  border: calc(8px * var(--scale)) solid red;
  margin: calc(10px * var(--scale));
  display: inline-block;
  font-size: calc(16px * var(--scale));
}
.box-rem {
  width: calc(60px * var(--scale));
  height: calc(60px * var(--scale));
  border: calc(0.5rem * var(--scale)) solid blue;
  margin: calc(10px * var(--scale));
  display: inline-block;
  font-size: calc(1rem * var(--scale));
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
