# css/css-grid/abspos/grid-positioned-items-and-autofit-tracks-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-positioned-items-and-autofit-tracks-003.html"
}
```

## style[0]

```css

.container {
  width: 200px;
}
.grid {
  position: relative;
  grid: 10px / repeat(auto-fit, 30px);
}
span {
  background: blue;
}
.abs {
  position: absolute;
  top:0; right:0; bottom:0; left:0;
  background: pink;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
