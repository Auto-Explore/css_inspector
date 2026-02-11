# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-021.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-021.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    flow-tolerance: 0;
    grid-template-columns: repeat(auto-fit, 50px);
    height: 200px;
    width: 500px;
    gap: 10px;
}

.grid-lanes > div {
    width: 100%;
    height: 100px;
    background-color: orange;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
