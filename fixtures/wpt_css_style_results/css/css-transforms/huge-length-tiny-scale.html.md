# css/css-transforms/huge-length-tiny-scale.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/huge-length-tiny-scale.html"
}
```

## style[0]

```css

body {
  overflow: hidden;
}
div {
  will-change: transform;
  transform: scale(0.005);
  transform-origin: 0 0;
  width: 40000px;
  height: 40000px;
  background: green;
  border: 2000px solid blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
