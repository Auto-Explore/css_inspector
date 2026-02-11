# css/css-gaps/grid/grid-gap-decorations-034.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-034.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }
    .grid-container {
        display: grid;
        grid-gap: 10px;
        grid-template-columns: 100px 100px 100px;
        grid-template-rows: 100px 100px 100px;
        width: 130px;
        height: 130px;
        overflow: hidden;
        column-rule-color: blue;
        column-rule-style: solid;
        column-rule-width: 5px;
        row-rule-color: gold;
        row-rule-style: solid;
        row-rule-width: 5px;
        background: pink;
    }
    .item {
        background: gray;
    }
```

```json
{
  "errors": 4,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
