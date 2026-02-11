# css/css-gaps/animation/row-rule-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/row-rule-width-interpolation.html"
}
```

## style[0]

```css

    .parent {
      row-rule-style: solid;
      row-rule-width: 90px;
    }

    .target {
      display: flex;
      row-gap: 10px;
      row-rule: 10px solid black;
      column-rule: 10px solid black;
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
