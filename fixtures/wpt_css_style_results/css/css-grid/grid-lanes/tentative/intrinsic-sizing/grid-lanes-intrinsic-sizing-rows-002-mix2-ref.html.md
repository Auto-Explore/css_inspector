# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-002-mix2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-002-mix2-ref.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid;
  gap: 1px 2px;
  /* keep fixed values small enough for spanners to have an effect */
  grid-template-rows: 1.1ch auto 1.4ch 1fr;
  grid-auto-flow: column;
  border: 1px solid;
  padding: 0 1px 0 2px;
  vertical-align: top;
  height: min-content;
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
