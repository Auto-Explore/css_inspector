# css/css-writing-modes/sizing-orthogonal-percentage-margin-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthogonal-percentage-margin-001.html"
}
```

## style[0]

```css

.container {
  background: cyan;
  border: solid thick;
  height: 200px;
  width: 500px;
}
.element {
  background: magenta;
  font: 25px/1 Ahem;
  margin: 10%; /* This should be computed against the container's inline size (500px), so it should be 50px and element's height should be 100px. */
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
