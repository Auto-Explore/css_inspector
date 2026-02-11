# css/css-gaps/grid/grid-gap-decorations-056.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-056.html"
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
    row-rule: 6px solid red;
    column-rule: 6px solid blue;
    rule-inset: 0px;

    column-rule-visibility-items: between;
    row-rule-visibility-items: between;

    column-rule-break: intersection;
    row-rule-break: intersection;
  }
  .item {
    width: 100%;
    height: 100%;
    background: lightgray;
    opacity: 0.8;
  }
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
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
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
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
