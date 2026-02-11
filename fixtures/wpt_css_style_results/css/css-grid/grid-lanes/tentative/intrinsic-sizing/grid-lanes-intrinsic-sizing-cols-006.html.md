# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-006.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-006.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid-lanes;
  gap: 1px 2px;
  border: 1px solid;
  padding: 0 1px 0 2px;
  vertical-align: top;
  flow-tolerance: 0;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
