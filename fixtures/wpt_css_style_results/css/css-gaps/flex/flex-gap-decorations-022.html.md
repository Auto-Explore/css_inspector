# css/css-gaps/flex/flex-gap-decorations-022.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-022.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .flex-container {
    height: 110px;
    width: 120px;

    display: flex;

    column-gap: 20px;
    row-gap: 10px;

    column-rule-color: blue;
    column-rule-style: solid;
    column-rule-width: 20px;

    row-rule-color: gold;
    row-rule-style: solid;
    row-rule-width: 10px;

    row-rule-break: intersection;
    row-rule-inset: 0;

    flex-wrap: wrap;
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
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-inset”.",
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
