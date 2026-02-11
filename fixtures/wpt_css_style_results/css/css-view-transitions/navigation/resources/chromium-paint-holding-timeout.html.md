# css/css-view-transitions/navigation/resources/chromium-paint-holding-timeout.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/resources/chromium-paint-holding-timeout.html"
}
```

## style[0]

```css

@view-transition {
  navigation: auto;
}
.right {
  view-transition-name: target;
  width: 100px;
  height: 100px;
  position: relative;
  left: 300px;
  background: green;
}
::view-transition-group(*),
::view-transition-new(*),
::view-transition-old(*) {
  animation-play-state: paused;
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
