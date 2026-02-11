# css/css-contain/contain-inline-size-grid-auto-fit.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-inline-size-grid-auto-fit.html"
}
```

## style[0]

```css

.container {
  width: 100px;
}
.grid {
  display: grid;
  height: 100px;
  grid-template-columns: repeat(auto-fit, minmax(0, 1fr));
  contain: inline-size;
}
.grid-item {
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
