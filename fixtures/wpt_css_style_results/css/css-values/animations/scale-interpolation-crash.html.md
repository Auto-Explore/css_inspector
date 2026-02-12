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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
