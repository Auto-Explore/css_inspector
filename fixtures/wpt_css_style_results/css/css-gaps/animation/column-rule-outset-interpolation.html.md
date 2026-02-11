# css/css-gaps/animation/column-rule-outset-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/column-rule-outset-interpolation.html"
}
```

## style[0]

```css

    .parent {
      column-rule-style: solid;
      column-rule-edge-inset-start: 20px;
      column-rule-edge-inset-end: 20px;
      column-rule-interior-inset-start: 20px;
      column-rule-interior-inset-end: 20px;
    }

    .target {
      display: flex;
      gap: 10px;
      row-rule: 10px solid black;
      column-rule: 10px solid black;

      column-rule-break: intersection;
      column-rule-edge-inset-start: 5px;
      column-rule-edge-inset-end: 5px;
      column-rule-interior-inset-start: 5px;
      column-rule-interior-inset-end: 5px;
    }
  
```

```json
{
  "errors": 11,
  "messages": [
    {
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-end”.",
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
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-end”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
