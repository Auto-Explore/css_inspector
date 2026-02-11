# css/css-grid/grid-lanes/tentative/fragmentation/grid-lanes-fragmentation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/fragmentation/grid-lanes-fragmentation-003.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:10px/1 monospace; padding:0; margin:0;
}
wrapper {
  display: block;
  width: 600px;
  height: 600px;
  overflow: hidden;
}

.columns {
  columns: 3;
  column-fill: auto;
  background: lightgrey;
  margin-bottom: 15px;
}

.grid {
  display: grid-lanes;
  grid-lanes-direction: row;
  grid: 20px auto 30px / none;
  border: solid;
  border-width: 3px 1px 7px 5px;
  padding: 1px 3px 5px 7px;
  gap: 1px 5px;
}

.grid > * {
  background: cyan;
  min-width: 20px;
  min-height: 10px;
}
.grid > :nth-child(4n) {
  background: blue;
  width: 35px;
}
.grid > :nth-child(3n) {
  background: gray;
  width: 40px;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
