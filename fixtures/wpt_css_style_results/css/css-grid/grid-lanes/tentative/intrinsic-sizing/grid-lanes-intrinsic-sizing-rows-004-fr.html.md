# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-004-fr.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-004-fr.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid-lanes;
  grid-lanes-direction: row;
  gap: 1px 2px;
  grid-template-rows: 1fr 2fr 1fr 1fr;
  border: 1px solid;
  padding: 1px 0 2px 0;
  vertical-align: top;
  height: max-content;
}

item {
  /* allow for differing min-content and max-content sizes */
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
