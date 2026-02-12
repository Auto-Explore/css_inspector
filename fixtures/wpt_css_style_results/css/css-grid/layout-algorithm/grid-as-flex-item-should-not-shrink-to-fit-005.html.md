# css/css-grid/layout-algorithm/grid-as-flex-item-should-not-shrink-to-fit-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-as-flex-item-should-not-shrink-to-fit-005.html"
}
```

## style[0]

```css

    body { overflow: hidden; }
    .flexbox {
        display: flex;
        flex-flow: column nowrap;
        align-content: flex-start;
        width: 100px;
        height: 100px;
        background: red;
    }
    .grid {
        display: grid;
        align-items: start;
    }
    .gridItem {
        background: green;
        height: 100px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
