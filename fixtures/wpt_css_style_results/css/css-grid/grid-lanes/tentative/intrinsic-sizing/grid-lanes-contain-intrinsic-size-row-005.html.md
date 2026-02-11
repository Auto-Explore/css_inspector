# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-005.html"
}
```

## style[0]

```css

#grid {
  display: grid-lanes;
  grid-template-rows: repeat(2, 35px);
  contain: size;
  contain-intrinsic-block-size: 0;
  contain-intrinsic-inline-size: 10px;
  width: max-content;
  background: lightblue;
}
.item {
  background: lightgreen;
  opacity: 0.5;
  width: 20px;
  height: 20px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-block-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-inline-size”.",
      "severity": "Error"
    },
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
