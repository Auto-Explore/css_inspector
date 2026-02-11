# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-028-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-028-ref.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid;
    background: gray;
    flow-tolerance: 0;
    grid-template-columns: repeat(auto-fill, 1px);
    width: auto;
    height: auto;
}

.grid-lanes > div {
    width: 100%;
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
