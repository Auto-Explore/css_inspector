# css/motion/offset-path-shape-shape-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-shape-001-ref.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  width: 600px;
  height: 400px;
}
#box {
  background-color: green;
  transform: translate(550px, 150px) rotate(90deg);
  width: 100px;
  height: 100px;
  border-radius: 50% 50% 0 0;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
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
