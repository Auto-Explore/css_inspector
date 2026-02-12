# css/css-backgrounds/animations/background-size-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-size-interpolation.html"
}
```

## style[0]

```css

.parent {
  background-size: 100px 100px;
}
.target {
  width: 80px;
  height: 100px;
  display: inline-block;
  border: 10px solid black;
  background-repeat: no-repeat;
  background-image: url(../resources/stripes-100.png),
                    url(../resources/stripes-100.png),
                    url(../resources/blue-100.png),
                    url(../resources/green-100.png);
  background-position: left top, right top, left bottom, right bottom;
  background-size: 10px 10px;
}
.expected {
  margin-right: 10px;
  border-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
