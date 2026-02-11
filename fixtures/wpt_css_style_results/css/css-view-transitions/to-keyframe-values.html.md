# css/css-view-transitions/to-keyframe-values.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/to-keyframe-values.html"
}
```

## style[0]

```css

:root { view-transition-name: none }
#item {
  view-transition-name: item;
  width: 10px;
  height: 10px;
  position: relative;
}
.shifted {
  left: 20px;
}
::view-transition-group(*) {
  animation-play-state: paused;
}
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
