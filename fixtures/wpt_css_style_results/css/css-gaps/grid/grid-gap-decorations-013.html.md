# css/css-gaps/grid/grid-gap-decorations-013.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-013.html"
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
    grid-template-columns: 100px 100px 100px;
    height: 320px;

    column-rule-color: blue;
    column-rule-style: solid;
    column-rule-width: 5px;

    row-rule-color: red;
    row-rule-style: solid;
    row-rule-width: 5px;

    column-rule-inset: 5px;
    column-rule-break: intersection;
  }

  .item {
    background: gray;
    opacity: 0.5;
  }
```

```json
{
  "errors": 5,
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
      "message": "Unknown property “column-rule-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
