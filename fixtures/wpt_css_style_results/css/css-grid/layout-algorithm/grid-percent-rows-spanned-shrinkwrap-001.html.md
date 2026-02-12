# css/css-grid/layout-algorithm/grid-percent-rows-spanned-shrinkwrap-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-percent-rows-spanned-shrinkwrap-001.html"
}
```

## style[0]

```css

    .grid {
        display: grid;
        float: left;
        margin: 1em;
        grid-template-rows: auto 20% auto;
        grid-auto-flow: column;
        justify-content: center;
        border: solid silver;
        font: 20px/1 Ahem;
        color: transparent;
    }
    .grid > div {
        background: blue;
    }
    .b {
        grid-row: 3;
    }
    .c {
        grid-row: span 3;
    }

    .ref {
        grid-template-rows: 40px 20px 40px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
