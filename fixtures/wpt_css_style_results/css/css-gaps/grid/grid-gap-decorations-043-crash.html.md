# css/css-gaps/grid/grid-gap-decorations-043-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-043-crash.html"
}
```

## style[0]

```css

  .grid-container {
    display: grid;
    column-gap: 40px;

    grid-template: 1fr 1fr / 1fr;
    column-rule: solid blue;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
