# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-001-mix1.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-001-mix1.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid-lanes;
  grid-lanes-direction: row;
  gap: 1px 2px;
  grid-template-rows: 1fr 2fr min-content max-content;
  border: 1px solid;
  padding: 0 1px 0 2px;
  vertical-align: top;
}

item {
  /* smaller heights to fit into 800x600 */
  writing-mode: vertical-rl;
  text-orientation: sideways;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
