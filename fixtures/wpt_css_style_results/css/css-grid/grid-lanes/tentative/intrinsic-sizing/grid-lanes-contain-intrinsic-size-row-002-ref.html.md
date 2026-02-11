# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-002-ref.html"
}
```

## style[0]

```css

#grid {
  border: 3px solid black;
  display: grid;
  grid-template-rows: 1fr 2fr;
  grid-template-columns: repeat(2, min-content);
  contain-intrinsic-size: 80px 90px;
  contain: size;
  width: max-content;
  background: lightblue;
  grid-gap: 5px;
}
.item {
  background: lightgreen;
  width: 20px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
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
