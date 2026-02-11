# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-fit-content-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-fit-content-002-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-rows: repeat(7, fit-content(100px));
    grid-template-columns: auto;
    width: 300px;
    height: 300px;
    font: 15px/1 Ahem;
}

.grid > div {
    width: 100px;
    height: min-content;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
