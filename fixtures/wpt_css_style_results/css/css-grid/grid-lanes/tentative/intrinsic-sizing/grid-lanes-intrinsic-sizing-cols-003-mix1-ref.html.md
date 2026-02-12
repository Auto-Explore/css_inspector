# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-003-mix1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-cols-003-mix1-ref.html"
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
  grid-template-columns: 1fr 2fr min-content max-content;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
