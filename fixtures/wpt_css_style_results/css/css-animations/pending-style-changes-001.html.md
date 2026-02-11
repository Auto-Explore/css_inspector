# css/css-animations/pending-style-changes-001.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/pending-style-changes-001.html"
}
```

## style[0]

```css

@keyframes anim {}

.animate {
  animation: anim 10s;
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
