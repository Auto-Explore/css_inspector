# css/css-gaps/grid/grid-gap-decorations-031.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-031.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .grid-container {
    display: grid;
    grid-gap: 10px;
    grid-template-rows: 100px 100px 100px;
    grid-auto-flow: column;
    width: 120px;
    height: 120px;

    row-rule-color: red;
    row-rule-style: solid;
    row-rule-width: 5px;
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
