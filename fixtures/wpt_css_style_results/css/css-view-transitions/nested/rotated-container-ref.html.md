# css/css-view-transitions/nested/rotated-container-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/rotated-container-ref.html"
}
```

## style[0]

```css

:root { background: pink }
#target {
  width: 100px;
  height: 100px;
  position: absolute;
  left: 50px;
  top: 50px;
  background: lightblue;

  transform: translateX(100px) rotate(45deg);
  will-change: transform;
}
#item {
  width: 50px;
  height: 50px;
  background: blue;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
