# css/css-grid/grid-lanes/tentative/alignment/column-overflow-alignment-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-overflow-alignment-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
    grid-template-columns: 50px;
    justify-items: unsafe center;
    width: 50px;
}
.overflow-safe {
    justify-self: safe end;
    width: 75px;
    height: 50px;
    background-color: lightgreen;
}
.overflow-unsafe {
    justify-self: unsafe end;
    width: 75px;
    height: 50px;
    background-color: lightblue;
}
.small-item {
    width: 25px;
    background-color: lightyellow;
}
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-items”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
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
