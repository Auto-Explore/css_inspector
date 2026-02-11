# css/css-gaps/grid/grid-gap-decorations-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-001.html"
}
```

## style[0]

```css

  .grid-container {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
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
