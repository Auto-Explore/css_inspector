# css/css-grid/grid-lanes/tentative/parsing/grid-lanes-shorthand-valid.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/parsing/grid-lanes-shorthand-valid.html"
}
```

## style[0]

```css

  #div {
    grid-lanes: "test" max-content row !important;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “grid-lanes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
