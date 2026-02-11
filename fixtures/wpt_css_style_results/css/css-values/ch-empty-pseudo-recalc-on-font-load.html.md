# css/css-values/ch-empty-pseudo-recalc-on-font-load.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ch-empty-pseudo-recalc-on-font-load.html"
}
```

## style[0]

```css

.before::before,
.after::after,
.backdrop::backdrop {
  font: 25px/1 "custom font", monospace;
  background: linear-gradient(45deg, red, blue);
  background-size: 1ch 1ch;
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
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
