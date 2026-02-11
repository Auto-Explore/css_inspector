# css/css-values/animations/scale-interpolation-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-values/animations/scale-interpolation-crash.html"
}
```

## style[0]

```css

body { animation: foo 1s; }
@keyframes foo { to { scale: calc(100% * 1); } }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
