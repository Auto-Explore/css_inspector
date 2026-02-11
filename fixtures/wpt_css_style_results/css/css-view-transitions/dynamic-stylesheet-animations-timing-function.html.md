# css/css-view-transitions/dynamic-stylesheet-animations-timing-function.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/dynamic-stylesheet-animations-timing-function.html"
}
```

## style[0]

```css

body { margin: 0; }
:root { view-transition-name: none; }
html::view-transition-group(*),
html::view-transition-old(*),
html::view-transition-new(*) {
  animation-duration: 10s;
  animation-delay: -5s;
  animation-play-state: paused;
}
#target { view-transition-name: target; }
.init {
  width: 100px;
  height: 100px;
}
.large {
  width: 300px;
  height: 300px;
}
.left {
  margin-left: 100px;
}
.right {
  margin-left: 300px;
}
/* For generating the transform with ease, as reference */
@keyframes anim {
  from { transform: translate(100px); }
  to { transform: translate(300px); }
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
