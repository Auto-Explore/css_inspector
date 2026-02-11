# css/css-anchor-position/anchor-function-in-calc-number-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-function-in-calc-number-crash.html"
}
```

## style[0]

```css

.anchor {
  anchor-name: --a;
  width: 100px;
  height: 100px;
  background: magenta;
}
.positioned {
  position: absolute;
  position-anchor: --a;
  width: 10px;
  height: 10px;
  background: purple;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
