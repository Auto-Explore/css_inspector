# css/css-animations/crashtests/pseudo-element-animation-with-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/crashtests/pseudo-element-animation-with-marker.html"
}
```

## style[0]

```css

@keyframes test { 0% { marker: url("crash"); } }
body:before { animation-name: test; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “marker”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
