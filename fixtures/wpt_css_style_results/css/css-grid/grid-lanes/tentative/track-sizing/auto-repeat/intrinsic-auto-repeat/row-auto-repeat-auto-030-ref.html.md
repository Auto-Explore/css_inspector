# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-030-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-030-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-rows: repeat(1, auto auto 100px);
    grid-template-columns: repeat(4, 100px);
    height: 900px;
    width: 300px;
}

.grid > div {
    height: 300px;
    width: 100px;
    grid-row: span 2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
