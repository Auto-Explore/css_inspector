# css/css-gaps/grid/grid-gap-decorations-063.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-063.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .grid-container {
    display: grid;
    grid-template-columns: 100px;
    grid-template-rows: repeat(auto-fit, 20px);
    row-gap: 10px;
    height: 260px;
    width: 100px;

    row-rule-color: red, blue, green, yellow, purple, teal, pink, gold, cornflowerblue, orange;
    row-rule-style: solid;
    row-rule-width: 4px;
  }

  .item {
    background: gray;
    opacity: 0.5;
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
