# css/css-view-transitions/inline-child-with-overflow-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/inline-child-with-overflow-shadow.html"
}
```

## style[0]

```css

:root { view-transition-name: none; }
::view-transition { background: rebeccapurple; }
body {
  margin: 0;
}

.target {
  width: 100px;
  height: 200px;
  position: absolute;
  top: 100px;
  left: 100px;
  view-transition-name: target;
}

.child {
  background: green;
  font-size: 100px;
  box-shadow: -20px -20px yellow;
}

html::view-transition-group(target) {
  animation-play-state: paused;
 }
html::view-transition-old(target),
html::view-transition-new(target) {
  animation: unset;
  opacity: 1;
}

html::view-transition-old(target) {
  opacity: 0;
}

/* None of these should apply, so make everything red if it does */
html::view-transition-group(root) { animation: unset; opacity: 1; background: red; }
html::view-transition-image-pair(root) { visibility: hidden }
```

```json
{
  "errors": 9,
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
      "message": "Invalid value for property “background”.",
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
