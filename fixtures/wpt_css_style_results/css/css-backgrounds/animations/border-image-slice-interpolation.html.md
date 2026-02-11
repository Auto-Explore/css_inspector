# css/css-backgrounds/animations/border-image-slice-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/border-image-slice-interpolation.html"
}
```

## style[0]

```css

.parent {
  border-image-slice: 50%;
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  border: 25px;
  border-image-source: linear-gradient(45deg, red, blue, green);
  border-image-slice: 20%;
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
