# css/css-gaps/flex/flex-gap-decorations-021.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-021.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }
    .flex-container {
        position: absolute;
        top: 0;
        left: 0;
        height: 110px;
        width: 110px;
        display: flex;
        column-gap: 10px;
        row-gap: 10px;
        column-rule-color: blue;
        column-rule-style: solid;
        column-rule-width: 10px;
        row-rule-color: gold;
        row-rule-style: solid;
        row-rule-width: 10px;
        flex-wrap: wrap;
    }
    .flex-item {
        background: skyblue;
        width: 50px;
    }
    .overflow {
        position: absolute;
        top: 0;
        left: 0;
        overflow: hidden;
        height: 80px;
        width: 80px;
    }
```

```json
{
  "errors": 4,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
