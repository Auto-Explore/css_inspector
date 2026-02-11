# css/css-align/animation/column-gap-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-align/animation/column-gap-interpolation.html"
}
```

## style[0]

```css

.parent {
  column-gap: 90px;
}
.target {
  column-count: 2;
  column-gap: 10px;
}
.expected div {
  opacity: 0.7;
}
.target > div {
  height: 20px;
  background-color: black;
}
.target.expected > div {
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
