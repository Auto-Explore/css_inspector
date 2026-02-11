# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-015.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-015.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    grid-lanes-direction: row;
    grid-template-rows: repeat(auto-fit, 100px);
    height: 300px;
    width: 300px;
}

.grid-lanes > div {
    height: 100%;
    width: 100px;
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
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
