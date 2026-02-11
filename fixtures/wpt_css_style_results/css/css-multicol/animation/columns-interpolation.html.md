# css/css-multicol/animation/columns-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/animation/columns-interpolation.html"
}
```

## style[0]

```css

  .parent {
    columns: 50px / 100px;
  }
  .target {
    columns: 200px / 100px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
