# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-023.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-023.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    grid-template-columns: 100px repeat(auto-fit, 100px);
    height: 500px;
    width: 300px;
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
