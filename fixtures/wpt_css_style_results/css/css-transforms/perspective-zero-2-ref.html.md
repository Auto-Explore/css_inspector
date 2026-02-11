# css/css-transforms/perspective-zero-2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-zero-2-ref.html"
}
```

## style[0]

```css

.parent {
  transform: perspective(0px);
  transform-style: preserve-3d;
  transform-origin: top left;
}
.parent > div {
  width: 100px;
  height: 100px;
  position: absolute;
}
.child-3d {
  background: green;
  transform: translateZ(0.5px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
