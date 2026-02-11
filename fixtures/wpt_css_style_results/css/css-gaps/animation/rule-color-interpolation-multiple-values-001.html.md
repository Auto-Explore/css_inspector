# css/css-gaps/animation/rule-color-interpolation-multiple-values-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-color-interpolation-multiple-values-001.html"
}
```

## style[0]

```css

    .parent {
      row-rule-style: solid;
      row-rule-color: white;
    }

    .target {
      display: flex;
      row-gap: 10px;
      row-rule-width: 10px;
      row-rule-style: solid;
      row-rule-color: black, black, black;
      flex-wrap: wrap;
      column-rule-color: 10px;
      column-rule-style: solid;
      column-rule-color: black, red, blue;
    }
  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
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
      "message": "Invalid value for property “column-rule-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
