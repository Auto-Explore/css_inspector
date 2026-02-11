# css/css-gaps/grid/grid-gap-decorations-repaint-on-item-span-change-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-repaint-on-item-span-change-ref.html"
}
```

## style[0]

```css

    .grid-container {
      display: grid;
      grid-template: repeat(2, 50px) / repeat(2, 50px);
      gap: 20px;
      column-rule: 20px solid green;
      row-rule: 20px solid green;
      rule-break: intersection;
      rule-interior-inset: -20px;
      rule-visibility-items: around;
    }

    .grid-item {
      background: green;
    }
  
```

```json
{
  "errors": 5,
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
      "message": "Unknown property “rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-interior-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “rule-visibility-items”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
