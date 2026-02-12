# css/css-transforms/animation/perspective-origin-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/perspective-origin-interpolation.html"
}
```

## style[0]

```css

.parent {
  perspective-origin: 30px 10px;
}
.target {
  display: inline-block;
  perspective: 50;
  margin-top: 50px;
  margin-bottom: 25px;
  perspective-origin: 10px 30px;
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
