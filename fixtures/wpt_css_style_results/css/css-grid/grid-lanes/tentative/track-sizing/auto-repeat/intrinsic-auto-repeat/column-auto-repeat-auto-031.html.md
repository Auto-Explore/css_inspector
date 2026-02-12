# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-031.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-031.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    grid-template-columns: repeat(auto-fill, auto auto 100px);
    width: 900px;
    height: 300px;
}

.grid-lanes > div {
    width: 300px;
    height: 100px;
    grid-column: span 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
