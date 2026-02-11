# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-024.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-024.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    flow-tolerance: 0;
    grid-lanes-direction: row;
    grid-template-rows: repeat(4, 50px) repeat(auto-fit, 50px) repeat(4, 50px);
    width: 200px;
    height: 500px;
    gap: 10px;
}

.grid-lanes > div {
    height: 100%;
    width: 100px;
    background-color: orange;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “grid-lanes-direction”.",
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
