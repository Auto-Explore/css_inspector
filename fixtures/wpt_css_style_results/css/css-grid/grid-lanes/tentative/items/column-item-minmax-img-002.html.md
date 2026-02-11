# css/css-grid/grid-lanes/tentative/items/column-item-minmax-img-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/column-item-minmax-img-002.html"
}
```

## style[0]

```css

.grid-lanes {
    display: inline-grid-lanes;
    grid-template-columns: minmax(auto, 0);
    border: 5px solid goldenrod;
    vertical-align: top;
}
.spacer{
    height: 10px;
}
img {
    border: 2px solid indigo;
}
.stretch {
    align-self: stretch;
    justify-self: stretch;
}
.start {
    align-self: start;
    justify-self: start;
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
