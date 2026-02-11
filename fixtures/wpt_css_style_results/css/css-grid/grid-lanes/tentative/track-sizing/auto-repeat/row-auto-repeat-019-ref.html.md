# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-019-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-019-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    grid-template-rows: repeat(auto-fit, 50px);
    grid-template-columns: 50px 50px;
    width: 200px;
    height: 500px;
    gap: 10px;
}

.grid > div {
    height: 100%;
    width: 50px;
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
