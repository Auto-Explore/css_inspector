# css/css-backgrounds/animations/border-image-source-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/border-image-source-interpolation.html"
}
```

## style[0]

```css

.parent {
  border-image-source: url(../support/green.png);
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  border: 5px solid aqua;
  border-image-source: url(../support/blue_color.png);
  border-image-slice: 10%;
}
.expected {
  background-color: green;
  margin-right: 2px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
