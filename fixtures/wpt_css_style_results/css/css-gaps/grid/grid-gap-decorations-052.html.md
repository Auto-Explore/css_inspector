# css/css-gaps/grid/grid-gap-decorations-052.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-052.html"
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
        column-rule: 3px solid red;
        column-rule-break: intersection;
        column-rule-interior-inset-start: 0px;
        column-rule-interior-inset-end: -5px;
        row-rule-break: intersection;
        row-rule-edge-inset-end: 5px;
        row-rule-edge-inset-start: 1px;
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
  "errors": 9,
  "messages": [
    {
      "message": "Unknown property “row-rule”.",
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
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-start”.",
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
