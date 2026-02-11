# css/css-grid/layout-algorithm/references/grid-percent-cols-spanned-shrinkwrap-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/references/grid-percent-cols-spanned-shrinkwrap-001-ref.html"
}
```

## style[0]

```css

    .grid {
        display: grid;
        float: left;
        margin: 1em;
        grid-template-columns: 40px 20px 40px;
        align-content: center;
        border: solid silver;
        font: 20px/1 Ahem;
        color: transparent;
    }
    .grid > div {
        background: blue;
    }
    .b {
        grid-column: 3;
    }
    .c {
        grid-column: span 3;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
