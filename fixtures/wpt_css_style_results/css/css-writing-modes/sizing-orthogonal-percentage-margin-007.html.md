# css/css-writing-modes/sizing-orthogonal-percentage-margin-007.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthogonal-percentage-margin-007.html"
}
```

## style[0]

```css

.container {
  background: cyan;
  border: solid thick;
  height: 500px;
  width: 200px;
  writing-mode: vertical-lr;
}
.element {
  background: magenta;
  font: 25px/1 Ahem;
  margin: 10%; /* This should be computed against the container's inline size (500px), so it should be 50px and element's width should be 100px. */
  writing-mode: horizontal-tb;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
