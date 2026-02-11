# css/css-gaps/multicol/multicol-gap-decorations-025.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-025.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .container {
        border: 2px solid rgb(96 139 168);
        width: 200px;
        height: 130px;
        column-count: 3;
        column-width: 60px;
        column-height: 60px;
        column-gap: 10px;
        row-gap: 10px;
        column-rule-width: 10px;
        column-rule-style: solid;
        column-rule-color: blue;

        row-rule-width: 10px;
        row-rule-style: solid;
        row-rule-color: gold;
        row-rule-break: intersection;
        row-rule-inset: 0px;
        column-wrap: wrap;
    }

    p {
        background: rgb(96 139 168 / 0.2);
        height: 60px;
        margin: 0px;
    }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “column-height”.",
      "severity": "Error"
    },
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
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-wrap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
