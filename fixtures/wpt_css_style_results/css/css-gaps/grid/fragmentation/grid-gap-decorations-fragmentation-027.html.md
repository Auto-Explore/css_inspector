# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-027.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-027.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    position: relative;
    columns: 4;
    column-fill: auto;
    height: 200px;
    width: 1750px;
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
    column-rule-break: normal;
    column-rule-inset: 0px;
    row-rule-break: normal;
    row-rule-inset: 0px;
    column-rule-visibility-items: around;
    row-rule-visibility-items: around;
  }
  .grid-item {
    background-color: gray;
    opacity: 0.5;
  }
```

```json
{
  "errors": 9,
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
    },
    {
      "message": "Unknown property “column-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-visibility-items”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
