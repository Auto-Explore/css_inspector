# css/css-gaps/flex/flex-gap-decorations-024.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-024.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .flex-container {
    height: 110px;
    width: 110px;
    display: flex;
    column-gap: 10px;
    row-gap: 10px;
    column-rule: 10px solid pink !important;
    row-rule-color: green;
    row-rule-style: solid;
    row-rule-width: 10px;
    flex-wrap: wrap;
  }
  #container {
    column-rule: 9px dotted blue;
  }
  .flex-item {
    background: skyblue;
    width: 50px;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
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
      "message": "Invalid value for property “column-rule”.",
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
