# css/css-borders/tentative/border-shape/border-shape-double-shape-default.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-double-shape-default.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
.container {
  display: flex;
  gap: 40px;
  padding: 30px;
}
.box {
  width: 100px;
  height: 100px;
  background: lightblue;
  border: 20px solid rgba(255, 0, 0, 0.5);
  fill: rgba(0, 255, 0, 0.3);
}
/* Double shape - should default to border-box + padding-box */
#double-implicit {
  border-shape: circle(50%) circle(40%);
}
/* Explicit border-box should match */
#double-explicit {
  border-shape: circle(50%) border-box circle(40%) padding-box;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-shape”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-shape”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
