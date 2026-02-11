# css/css-view-transitions/pseudo-element-animations.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-element-animations.html"
}
```

## style[0]

```css

/* Override UA stylesheet to avoid any impact on our tests */
:root {
  view-transition-name: none;
}
:root::view-transition,
:root::view-transition-group(*),
:root::view-transition-image-pair(*),
:root::view-transition-old(*),
:root::view-transition-new(*) {
  animation: unset;
}

@keyframes css-anim {
  from { margin-left: 100px; }
  to { margin-left: 100px; }
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
