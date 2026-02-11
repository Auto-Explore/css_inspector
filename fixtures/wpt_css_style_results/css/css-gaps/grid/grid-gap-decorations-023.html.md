# css/css-gaps/grid/grid-gap-decorations-023.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-023.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
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
        column-rule-break: none;
        row-rule-break: none;
        rule-overlap: column-over-row;
    }
    .grid-item {
        background-color: gray;
        opacity: 0.5;
        border: 1px solid #000;
    }
```

```json
{
  "errors": 6,
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
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-overlap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
