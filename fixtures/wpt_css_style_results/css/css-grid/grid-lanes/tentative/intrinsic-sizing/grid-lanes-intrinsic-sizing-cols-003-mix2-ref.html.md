# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-003-mix2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-003-mix2-ref.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid;
  gap: 1px 2px;
  border: 1px solid;
  padding: 0 1px 0 2px;
  vertical-align: top;
  width: min-content;
  /* keep fixed values small enough for spanners to have an effect */
  grid-template-columns: 1.1ch auto 1.4ch 1fr;
}

item {
  align-self: start;
}

.hidden {
  visibility: hidden;
  height: 0;
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
