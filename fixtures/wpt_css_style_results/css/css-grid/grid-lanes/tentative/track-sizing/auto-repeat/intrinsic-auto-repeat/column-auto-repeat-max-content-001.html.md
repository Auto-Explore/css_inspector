# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-max-content-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-max-content-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    grid-template-columns: repeat(auto-fill, max-content);
    width: 300px;
    height: 300px;
    font: 15px/1 Ahem;
}

.grid-lanes > div {
    height: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
