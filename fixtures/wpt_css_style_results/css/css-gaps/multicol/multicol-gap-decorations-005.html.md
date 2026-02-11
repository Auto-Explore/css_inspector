# css/css-gaps/multicol/multicol-gap-decorations-005.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-005.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .outer-container {
        column-count: 2;
        column-gap: 10px;
        column-rule-width: 10px;
        column-rule-style: solid;
        column-rule-color: blue;
        border: 1px solid #ccc;
        width: 210px;
        height: 50px;
    }

    .nested-container {
        column-count: 2;
        column-gap: 10px;
        width: 100px;
        column-rule-width: 10px;
        column-rule-style: solid;
        column-rule-color: gold;
        row-gap: 10px;
        row-rule-width: 10px;
        row-rule-style: solid;
        row-rule-color: purple;
        column-wrap: wrap;
        column-fill: auto;
        column-height: 20px;
    }

    .column1 {
        background: rgb(96 139 168 / 0.2);
        width: 100px;
        height: 50px
    }

    .nested-column {
        background: black;
        width: 45px;
        height: 20px;
    }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-wrap”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-height”.",
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
