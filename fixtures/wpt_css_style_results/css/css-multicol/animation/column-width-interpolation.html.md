# css/css-multicol/animation/column-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/animation/column-width-interpolation.html"
}
```

## style[0]

```css

.parent {
  column-width: 30px;
}
.target {
  font-size: 0px; /* column-width "specified values must be greater than 0", so use font-size to achieve 0px computed value. */
  display: inline-block;
  column-width: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
