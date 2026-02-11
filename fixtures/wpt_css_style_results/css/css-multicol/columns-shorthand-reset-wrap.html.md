# css/css-multicol/columns-shorthand-reset-wrap.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/columns-shorthand-reset-wrap.html"
}
```

## style[0]

```css

  #multicol1 {
    columns: 100px / 100px;
  }
  #multicol2 {
    column-wrap: nowrap;
    columns: 100px / 100px;
  }
  #multicol3 {
    columns: 100px / 100px;
    column-wrap: nowrap;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-wrap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
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
