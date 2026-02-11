# css/css-transforms/animation/perspective-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/perspective-interpolation.html"
}
```

## style[0]

```css

.parent {
  perspective: 30px;
}
.target {
  perspective: 10px;
}
.transformed {
  width: 50px;
  height: 50px;
  background: black;
  transform: rotateY(45deg);
}
.expected .transformed {
  background: green;
}
.expected {
  position: relative;
  left: -50px;
  opacity: 0.75;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
