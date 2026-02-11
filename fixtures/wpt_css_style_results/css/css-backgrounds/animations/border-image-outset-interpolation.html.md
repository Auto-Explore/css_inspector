# css/css-backgrounds/animations/border-image-outset-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/border-image-outset-interpolation.html"
}
```

## style[0]

```css

.parent {
  border-image-outset: 10px;
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  border: 25px;
  margin-right: 50px;
  border-image-slice: 30%;
  background-clip: content-box;
  border-image-source: linear-gradient(45deg, pink, blue, white, black, green);
  border-image-outset: 1px;
}
.expected {
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
