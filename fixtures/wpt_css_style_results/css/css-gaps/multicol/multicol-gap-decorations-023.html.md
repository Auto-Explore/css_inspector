# css/css-gaps/multicol/multicol-gap-decorations-023.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-023.html"
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
        column-wrap: wrap;
        column-rule-break: intersection;
        column-rule-interior-inset-start: 0;
        column-rule-interior-inset-end: -8px;
        row-rule-edge-inset-start: -2px;
        row-rule-edge-inset-end: -1px;
        rule-overlap: column-over-row;
    }
    p {
        background: rgb(96 139 168 / 0.2);
        height: 60px;
        margin: 0px;
    }
```

```json
{
  "errors": 13,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
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
      "message": "Unknown property “column-wrap”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-overlap”.",
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
