# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-fit-content-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-fit-content-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-columns: repeat(3, fit-content(100px));
    grid-template-rows: repeat(3, 100px);
    width: 300px;
    height: 300px;
    font: 15px/1 Ahem;
}

.grid > div {
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
