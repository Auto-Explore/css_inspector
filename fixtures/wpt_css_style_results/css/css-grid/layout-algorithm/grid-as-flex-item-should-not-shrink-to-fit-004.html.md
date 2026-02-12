# css/css-grid/layout-algorithm/grid-as-flex-item-should-not-shrink-to-fit-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-as-flex-item-should-not-shrink-to-fit-004.html"
}
```

## style[0]

```css

    body { overflow: hidden; }
    .flexbox {
        display: flex;
        flex-flow: column wrap;
        align-content: stretch;
        width: 100px;
        height: 100px;
        background: red;
    }
    .grid {
        display: grid;
    }
    .gridItem {
        background: green;
        height: 100px;
        writing-mode: vertical-lr;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
