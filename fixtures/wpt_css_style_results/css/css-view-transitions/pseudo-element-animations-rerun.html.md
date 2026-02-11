# css/css-view-transitions/pseudo-element-animations-rerun.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-element-animations-rerun.html"
}
```

## style[0]

```css

:root::view-transition,
:root::view-transition-group(root),
:root::view-transition-image-pair(root),
:root::view-transition-old(root),
:root::view-transition-new(root) {
    animation: view-transition-animation 1ms;
}
@keyframes view-transition-animation {
    to { opacity: 0 }
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
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
