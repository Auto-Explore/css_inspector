# css/css-multicol/animation/column-wrap-reset-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/animation/column-wrap-reset-interpolation.html"
}
```

## style[0]

```css

  .multicol {
    columns: 100px / 100px;
    column-wrap: nowrap;
    transition: columns 2s allow-discrete;
  }
  #mid {
    transition-delay: -1s;
  }
  #end {
    transition-delay: -2s;
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
      "message": "Unknown property “column-wrap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
