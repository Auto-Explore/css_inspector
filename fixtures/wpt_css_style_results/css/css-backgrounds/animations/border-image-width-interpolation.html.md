# css/css-backgrounds/animations/border-image-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/border-image-width-interpolation.html"
}
```

## style[0]

```css

.parent {
  border-image-width: 100px;
}
.target {
  width: 80px;
  height: 80px;
  background-color: black;
  display: inline-block;
  border: 10px;
  border-image-source: linear-gradient(45deg, red, blue, green);
  border-image-width: 10px;
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
