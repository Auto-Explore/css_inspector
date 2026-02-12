# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-001-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-001-auto-ref.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid;
  gap: 1px 2px;
  grid-template-rows: repeat(4,auto);
  grid-auto-flow: column;
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
