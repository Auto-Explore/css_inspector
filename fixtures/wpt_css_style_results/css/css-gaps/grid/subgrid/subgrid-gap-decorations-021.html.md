# css/css-gaps/grid/subgrid/subgrid-gap-decorations-021.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-021.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .grid {
    display: grid;
    grid-template: repeat(3, 100px) / repeat(3, 100px);
    gap: 20px;
  }
  .subgrid {
    display: grid;
    grid-template: subgrid / subgrid;
    grid-row: 1/-1;
    grid-column: 1/-1;
    column-rule: 6px solid blue;
    row-rule: 6px solid red;
    rule-inset: 0px;
    column-rule-visibility-items: between;
    row-rule-visibility-items: between;
    rule-break: intersection;
  }
  .item {
    width: 100%;
    height: 100%;
    background: lightgrey;
    opacity: 0.8;
  }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-break”.",
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
