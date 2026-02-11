# css/motion/offset-path-shape-inset-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-inset-002-ref.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  width: 400px;
  height: 400px;
}
#box {
  background-color: green;
  transform: translate(306px, 94px) rotate(45deg);
  transform-origin: 0 0;
  border-radius: 50% 50% 0 0;
  width: 100px;
  height: 100px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
