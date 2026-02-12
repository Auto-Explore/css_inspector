# css/css-masking/animations/mask-image-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/mask-image-interpolation.html"
}
```

## style[0]

```css

.parent {
  mask-image: url(../resources/blue-20.png);
}
.target {
  width: 20px;
  height: 20px;
  display: inline-block;
  background-color: black;
  mask-image: url(../resources/stripes-20.png);
}
.expected {
  background-color: green;
  margin-right: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
