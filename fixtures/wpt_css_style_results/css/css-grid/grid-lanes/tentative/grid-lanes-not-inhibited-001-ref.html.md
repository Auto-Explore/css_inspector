# css/css-grid/grid-lanes/tentative/grid-lanes-not-inhibited-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-lanes-not-inhibited-001-ref.html"
}
```

## style[0]

```css

grid {
  vertical-align: top;
  display: inline-grid;
  grid-template-columns: 60px 60px;
  grid-template-rows: 60px;
  border: 2px solid black;
}
item {
  width: 60px;
  height: 60px;
  background: cyan;
}
.tall {
  background: tan;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
