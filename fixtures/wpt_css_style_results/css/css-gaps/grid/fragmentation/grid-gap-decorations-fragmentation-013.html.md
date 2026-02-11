# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-013.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-013.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    height: 215px;
    width: 320px;
    columns: 3;
    column-fill: auto;
    column-gap: 10px;
    background: lightgray;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(13, 10px);
    column-gap: 10px;
    row-gap: 10px;
    row-rule: solid 5px red;
    column-rule: solid 6px blue;
  }
  .grid-container>div {
    background-color: skyblue;
  }
```

```json
{
  "errors": 4,
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
      "message": "Invalid value for property “column-rule”.",
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
