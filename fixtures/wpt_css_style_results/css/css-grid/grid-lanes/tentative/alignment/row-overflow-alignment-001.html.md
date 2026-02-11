# css/css-grid/grid-lanes/tentative/alignment/row-overflow-alignment-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/row-overflow-alignment-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    background: gray;
    flow-tolerance: 0;
    grid-template-rows: 70px;
    align-items: unsafe center;
    width: 300px;
    height: 70px;
}
.overflow-safe {
    align-self: safe end;
    width: 75px;
    height: 75px;
    background-color: lightgreen;
}
.overflow-unsafe {
    align-self: unsafe end;
    width: 75px;
    height: 75px;
    background-color: lightblue;
}
.small-item {
    width: 50px;
    background-color: lightyellow;
}
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-items”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
