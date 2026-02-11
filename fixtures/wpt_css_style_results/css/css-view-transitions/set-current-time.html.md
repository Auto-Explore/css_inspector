# css/css-view-transitions/set-current-time.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/set-current-time.html"
}
```

## style[0]

```css

:root { view-transition-name: unset; }
.target {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
}

html::view-transition-group(*) {
  animation: unset;
}
html::view-transition-old(*) {
  animation: unset;
}
html::view-transition-new(target) {
  animation-duration: 1s;
  animation-timing-function: linear;
  animation-play-state: paused;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
