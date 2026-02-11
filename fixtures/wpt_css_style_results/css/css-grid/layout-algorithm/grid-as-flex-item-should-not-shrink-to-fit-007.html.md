# css/css-grid/layout-algorithm/grid-as-flex-item-should-not-shrink-to-fit-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-as-flex-item-should-not-shrink-to-fit-007.html"
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
        align-items: baseline;
        grid-template-rows: 100px;
    }
    .gridItem {
        background: green;
        height: 100%;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
