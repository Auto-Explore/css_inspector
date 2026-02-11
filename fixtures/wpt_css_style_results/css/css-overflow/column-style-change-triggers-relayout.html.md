# css/css-overflow/column-style-change-triggers-relayout.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/column-style-change-triggers-relayout.html"
}
```

## style[0]

```css

#container {
  scroll-snap-type: both mandatory;
  overflow: hidden;
  columns: 1;
  gap: 0;
  column-fill: auto;
  width: 100px;
  height: 100px;
  background: red;
}
.snapColumns::column {
  scroll-snap-align: start;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
