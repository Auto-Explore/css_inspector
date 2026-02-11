# css/motion/offset-path-shape-inset-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-inset-001-ref.html"
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
  transform: translate(348px, 340px) rotate(180deg);
  border-radius: 50% 50% 0 0;
  width: 100px;
  height: 100px;
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
