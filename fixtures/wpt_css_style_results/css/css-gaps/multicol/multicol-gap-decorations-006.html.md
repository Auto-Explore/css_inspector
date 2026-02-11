# css/css-gaps/multicol/multicol-gap-decorations-006.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-006.html"
}
```

## style[0]

```css

    .outer-container {
        columns: 3;
        column-fill: auto;
        height: 100px;
        width: 620px;
        gap: 10px;
        background: yellow;
    }

    .nested-container {
        columns: 2;
        box-decoration-break: clone;
        padding: 10px;
        column-rule: solid;
        column-rule-color: green;
        column-rule-width: 1px;
        background: cyan;
        gap: 10px;
    }

    .inner-column {
        height: 250px;
        background: hotpink;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
