# css/css-view-transitions/style-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/style-inheritance.html"
}
```

## style[0]

```css

.paused::view-transition-group(*) {
  animation-play-state: paused;
}

::view-transition {
  background-color: red;
}

::view-transition-group(*) {
  background-color: inherit;
  color: blue;
  animation-duration: 0.321s;
  animation-delay: 0.05s;
  animation-iteration-count: 2;
  animation-direction: reverse;
  animation-timing-function: linear;
}

::view-transition-image-pair(*) {
  color: inherit;
  overflow-x: clip;
}

::view-transition-old(*), ::view-transition-new(*) {
  overflow-x: inherit;
}
```

```json
{
  "errors": 5,
  "messages": [
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
