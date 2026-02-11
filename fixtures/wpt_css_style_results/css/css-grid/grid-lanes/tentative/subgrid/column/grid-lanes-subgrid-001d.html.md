# css/css-grid/grid-lanes/tentative/subgrid/column/grid-lanes-subgrid-001d.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/subgrid/column/grid-lanes-subgrid-001d.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}

grid {
  display: inline-grid-lanes;
  grid-lanes-direction: row;
  grid-template-rows: 50px 80px 40px;
  gap: 4px 2px;
  padding: 1px 3px 5px 7px;
  border: solid;
  border-width: 3px 5px 1px 1px;
  background: lightgrey content-box;
}
.rows {
  grid-template-columns: 50px 80px 40px;
}
item {
  background: grey;
  width: 3ch;
  position: relative;
}
item:nth-child(2n) { background:purple; width:auto; }
item:nth-child(1) {
  border: solid;
  border-width: 3px 13px 1px 1px;
  margin: 7px 1px 5px 3px;
}
subgrid {
  display: grid;
  grid: subgrid / subgrid;
  grid-row: 2 / span 2;
  grid-gap: 8px 20px;
  background: yellow;
}
.rows > subgrid {
  grid-row: initial;
  grid-column: 2 / span 2;
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
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
