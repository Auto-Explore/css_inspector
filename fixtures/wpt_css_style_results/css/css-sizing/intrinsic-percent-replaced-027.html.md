# css/css-sizing/intrinsic-percent-replaced-027.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/intrinsic-percent-replaced-027.html"
}
```

## style[0]

```css

.outer {
  display: flex;
  flex-direction: column;
}
.inner {
  display: flex;
  width: 100px;
  height: 100px;
}
.test {
  aspect-ratio: 1 / 1;
  height: 100%;
  background: red;
}
.test > canvas {
  height: 100%;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
