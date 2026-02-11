# css/css-viewport/zoom/perspective-small-effective-zoom-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/perspective-small-effective-zoom-crash.html"
}
```

## style[0]

```css

@keyframes kf {
  from { transform: perspective(10px) }
}
* {
  zoom: 10%;
  animation: kf 1s;
}
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
