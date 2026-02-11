# css/css-gaps/flex/flex-gap-decorations-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-001.html"
}
```

## style[0]

```css

  .flex-container {
    display: flex;
    column-gap: 10px;
    row-gap: 10px;
    height: 100px;
    width: 100px;
    background: red;
    column-rule-color: green;
    column-rule-style: solid;
    column-rule-width: 10px;
    row-rule-color: green;
    row-rule-style: solid;
    row-rule-width: 10px;

    flex-wrap: wrap;
  }

  .flex-item {
    background: green;
    width: 45px;
  }
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
