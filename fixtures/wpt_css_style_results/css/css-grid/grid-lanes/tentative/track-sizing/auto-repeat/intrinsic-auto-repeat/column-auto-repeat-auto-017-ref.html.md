# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-017-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-017-ref.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-template-columns: repeat(9, auto);
    height: 200px;
    width: 500px;
    gap: 10px;
}

.grid-lanes > div {
    width: 50px;
    height: 100px;
    background-color: orange;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
