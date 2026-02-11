# css/filter-effects/background-image-blur-repaint.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/background-image-blur-repaint.html"
}
```

## style[0]

```css

.bg {
  position: absolute;
  left:   200px;
  top:    0px;
  width:  400px;
  height: 300px;
  background-image: url(support/color-palette.png);
  filter: blur(8px);
}
.box {
  position: absolute;
  left:   300px;
  top:    50px;
  width:  100px;
  height: 100px;
  background-color: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
