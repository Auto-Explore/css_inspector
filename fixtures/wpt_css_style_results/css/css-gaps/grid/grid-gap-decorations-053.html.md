# css/css-gaps/grid/grid-gap-decorations-053.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-053.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }
    .grid {
        display: grid;
        grid-template: repeat(auto-fill, 100px) / repeat(3, 100px);
        grid-gap: 20px;
        row-rule: 3px solid gray;
        row-rule-break: intersection;
        row-rule-interior-inset-start: 0;
        row-rule-interior-inset-end: 1px;
        column-rule: 3px solid red;
        column-rule-break: intersection;
        column-rule-interior-inset-start: 0;
        column-rule-interior-inset-end: -8px;
        column-rule-edge-inset-start: 1px;
        column-rule-edge-inset-end: -10px;
        left: 0px;
        top: 0px;
    }
    .item {
        width: 100px;
        height: 100px;
        background: lightgray;
    }
```

```json
{
  "errors": 11,
  "messages": [
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-end”.",
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
      "message": "Unknown property “column-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-end”.",
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
