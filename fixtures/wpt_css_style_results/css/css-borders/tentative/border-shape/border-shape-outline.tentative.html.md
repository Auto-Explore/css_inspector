# css/css-borders/tentative/border-shape/border-shape-outline.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-outline.tentative.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
.container {
  padding: 20px;
}
.target {
  width: 100px;
  height: 100px;
  background: lightblue;
  border-shape: circle(50px at 50px 50px);
  outline: 4px solid red;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
