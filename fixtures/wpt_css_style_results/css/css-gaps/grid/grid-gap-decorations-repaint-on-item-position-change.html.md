# css/css-gaps/grid/grid-gap-decorations-repaint-on-item-position-change.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-repaint-on-item-position-change.html"
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

    #walking-item {
      /* Start at row 1, col 1; will move to row 2, col 2 */
      grid-row: 1;
      grid-column: 1;
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
