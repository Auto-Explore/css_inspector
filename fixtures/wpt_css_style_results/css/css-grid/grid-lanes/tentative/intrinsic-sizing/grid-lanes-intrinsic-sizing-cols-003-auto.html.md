# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-003-auto.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-003-auto.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid-lanes;
  gap: 1px 2px;
  grid-template-columns: repeat(4,auto);
  border: 1px solid;
  padding: 0 1px 0 2px;
  vertical-align: top;
  width: min-content;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
