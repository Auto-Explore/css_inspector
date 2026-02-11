# css/css-gaps/grid/grid-gap-decorations-041-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-041-crash.html"
}
```

## style[0]

```css

  .grid-container {
    display: grid;

    column-rule: solid blue;
    row-rule: dotted red;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
