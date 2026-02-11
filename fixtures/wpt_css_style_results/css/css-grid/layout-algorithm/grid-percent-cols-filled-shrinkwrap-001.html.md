# css/css-grid/layout-algorithm/grid-percent-cols-filled-shrinkwrap-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-percent-cols-filled-shrinkwrap-001.html"
}
```

## style[0]

```css

    .grid {
        display: grid;
        float: left;
        margin: 1em;
        grid-template-columns: auto 20% auto;
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
        grid-column: 2;
    }

    .ref {
        grid-template-columns: 40px 20px 40px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
