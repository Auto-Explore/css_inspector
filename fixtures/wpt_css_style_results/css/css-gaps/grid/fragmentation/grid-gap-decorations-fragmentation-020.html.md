# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-020.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-020.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    position: relative;
    columns: 2;
    column-fill: auto;
    height: 310px;
    width: 870px;
    column-gap: 10px;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;

    width: 430px;
    height: 430px;

    column-rule-color: blue;
    column-rule-style: solid;
    column-rule-width: 5px;

    row-rule-color: red;
    row-rule-style: solid;
    row-rule-width: 5px;

    column-rule-break: intersection;
    column-rule-inset: 0px;

    row-rule-break: intersection;
    row-rule-inset: 0px;
  }
  .grid-item {
    background-color: gray;
    opacity: 0.5;
  }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
