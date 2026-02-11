# css/css-gaps/grid/grid-gap-decorations-022.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-022.html"
}
```

## style[0]

```css

  .grid-container {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
    height: 100px;
    width: 100px;
    background: red;
    rule-color: green;
    rule-width: 10px;
    rule-style: solid;
  }

  .grid-item {
    background: green;
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
