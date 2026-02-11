# css/css-gaps/flex/flex-gap-decorations-019.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-019.html"
}
```

## style[0]

```css

  .flex-container {
    display: flex;
    gap: 10px;
    height: 100px;
    width: 100px;
    background: red;
    rule-color: green;
    rule-width: 10px;
    rule-style: solid;

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
      "message": "Unknown property “rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
