# css/css-writing-modes/sizing-orthogonal-percentage-margin-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthogonal-percentage-margin-005-ref.html"
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
  margin: 20px;
  height: 460px;
  writing-mode: vertical-lr;
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
