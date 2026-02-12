# css/css-writing-modes/sizing-orthogonal-percentage-margin-004.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthogonal-percentage-margin-004.html"
}
```

## style[0]

```css

.container {
  background: cyan;
  border: solid thick;
  height: 200px;
  width: 500px;
  writing-mode: vertical-rl;
}
.element {
  background: magenta;
  font: 50px/1 Ahem;
  margin: 10%; /* This should be computed against the container's inline size (200px), so it should be 20px and element's width should be 460px. */
  writing-mode: horizontal-tb;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
