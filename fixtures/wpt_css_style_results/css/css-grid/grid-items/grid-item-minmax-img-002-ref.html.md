# css/css-grid/grid-items/grid-item-minmax-img-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-item-minmax-img-002-ref.html"
}
```

## style[0]

```css

.grid {
    display: inline-grid;
    grid-template-columns: 4px;
    grid-template-rows: 4px;
    border: 5px solid goldenrod;
}
.spacer{
    height: 10px;
}
.img{
    background-image: url("support/100x100-green.png");
    width: 100px;
    height: 100px;
}
.img, .empty{
    border: 2px solid indigo;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
