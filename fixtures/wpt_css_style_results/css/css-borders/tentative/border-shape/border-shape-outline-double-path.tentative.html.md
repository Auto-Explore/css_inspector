# css/css-borders/tentative/border-shape/border-shape-outline-double-path.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-outline-double-path.tentative.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
.container {
  padding: 40px;
}
.target {
  width: 100px;
  height: 100px;
  background: lightblue;
  /* Double path: outer circle, inner smaller circle */
  /* This creates a ring/donut shape for the border */
  border-shape: circle(50px at 50px 50px) circle(30px at 50px 50px);
  border-color: rgba(255, 0, 0, 0.5);
  outline: 4px solid green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
