# css/css-writing-modes/sizing-orthogonal-percentage-margin-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthogonal-percentage-margin-003-ref.html"
}
```

## style[0]

```css

.container {
  background: cyan;
  border: solid thick;
  height: 200px;
  width: 500px;
  writing-mode: vertical-lr;
}
.element {
  background: magenta;
  font: 50px/1 Ahem;
  margin: 20px;
  width: 460px;
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
