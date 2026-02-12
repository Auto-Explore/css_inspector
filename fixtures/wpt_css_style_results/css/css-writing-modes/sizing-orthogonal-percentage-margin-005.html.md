# css/css-writing-modes/sizing-orthogonal-percentage-margin-005.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthogonal-percentage-margin-005.html"
}
```

## style[0]

```css

.container {
  background: cyan;
  border: solid thick;
  height: 500px;
  width: 200px;
}
.element {
  background: magenta;
  font: 50px/1 Ahem;
  margin: 10%; /* This should be computed against the container's inline size (200px), so it should be 20px and element's height should be 460px. */
  writing-mode: vertical-lr;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
