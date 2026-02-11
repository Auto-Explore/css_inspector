# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-004-fr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-004-fr-ref.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid;
  gap: 1px 2px;
  grid-template-rows: 1fr 2fr 1fr 1fr;
  grid-auto-flow: column;
  border: 1px solid;
  padding: 1px 0 2px 0;
  vertical-align: top;
  height: max-content;
}

item {
  justify-self: start;
  writing-mode: vertical-rl;
  text-orientation: sideways;
}

.hidden {
  visibility: hidden;
  opacity: 0.5;
  width: 1em;
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
