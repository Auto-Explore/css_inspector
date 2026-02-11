# css/css-transforms/preserve3d-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-pseudo-element.html"
}
```

## style[0]

```css

div {
  width: 200px;
  height: 200px;
  transform: rotateX(90deg);
  transform-style: preserve-3d;
}

div::before {
  display: inline-block;
  width: 200px;
  height: 200px;
  transform: rotateX(90deg);
  content: '';
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
