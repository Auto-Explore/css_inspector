# css/css-gaps/animation/row-rule-outset-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/row-rule-outset-interpolation.html"
}
```

## style[0]

```css

    .parent {
      row-rule-style: solid;
      row-rule-edge-inset-start: 20px;
      row-rule-edge-inset-end: 20px;
      row-rule-interior-inset-start: 20px;
      row-rule-interior-inset-end: 20px;
    }

    .target {
      display: flex;
      row-gap: 10px;
      row-rule: 10px solid black;
      column-rule: 10px solid black;

      row-rule-break: intersection;
      row-rule-edge-inset-start: 5px;
      row-rule-edge-inset-end: 5px;
      row-rule-interior-inset-start: 5px;
      row-rule-interior-inset-end: 5px;
    }
  
```

```json
{
  "errors": 12,
  "messages": [
    {
      "message": "Unknown property “row-rule-style”.",
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
      "message": "Unknown property “row-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-end”.",
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
      "message": "Unknown property “row-rule-break”.",
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
      "message": "Unknown property “row-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-end”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
