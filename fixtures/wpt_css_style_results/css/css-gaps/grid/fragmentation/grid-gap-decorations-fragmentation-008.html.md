# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-008.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-008.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    height: 100px;
    width: 320px;
    columns: 3;
    column-fill: auto;
    column-gap: 10px;
    background: lightgray;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(4, 50px);
    column-gap: 10px;
    row-gap: 20px;
    row-rule: solid 10px red;
    row-rule-break: intersection;
    row-rule-edge-inset-start: 1px;
    column-rule: solid 6px blue;
    column-rule-break: intersection;
    column-rule-edge-inset-start: 1px;
    column-rule-interior-inset-end: 0px;
  }
  .grid-container>div {
    background-color: skyblue;
  }
```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-end”.",
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
