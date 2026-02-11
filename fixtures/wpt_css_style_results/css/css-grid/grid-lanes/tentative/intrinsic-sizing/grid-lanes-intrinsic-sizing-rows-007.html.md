# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-007.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: grid-lanes;
  grid-lanes-direction: row;
  gap: 1rem 1rem;
  grid-template-rows: repeat(4,100px);
  border: 1px solid;
  padding: 0 1px 0 2px;
  vertical-align: top;
}

item {
    background-color: blue;
    width: auto;
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
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
