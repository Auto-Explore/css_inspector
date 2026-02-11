# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-002.html"
}
```

## style[0]

```css

#border {
  width: max-content;
  border: 1px solid black;
}
#target {
  display: grid-lanes;
  background: lightblue;
  contain-intrinsic-size: 111px 222px;
  contain: size;
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
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
