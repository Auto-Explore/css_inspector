# css/css-view-transitions/rotated-cat-off-top-edge.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/rotated-cat-off-top-edge.html"
}
```

## style[0]

```css

.target {
  transform: rotate(330deg) translate3d(110px, 10px, 10px);
  border-radius: 0.375rem;
  width: 400px;
  height: 400px;
}
::view-transition-group(root) {
  animation-duration: 500s;
  animation-play-state: paused;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}

::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
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
