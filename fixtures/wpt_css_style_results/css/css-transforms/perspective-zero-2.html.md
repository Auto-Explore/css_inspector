# css/css-transforms/perspective-zero-2.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-zero-2.html"
}
```

## style[0]

```css

.parent {
  perspective: 0px;
  perspective-origin: top left;
}
.parent > div {
  position: absolute;
}
.child-2d {
  background: red;
  width: 200px;
  height: 200px;
}
.child-3d {
  width: 100px;
  height: 100px;
  background: green;
  transform: translateZ(0.5px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “perspective-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
