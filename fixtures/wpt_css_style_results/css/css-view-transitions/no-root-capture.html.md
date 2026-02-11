# css/css-view-transitions/no-root-capture.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/no-root-capture.html"
}
```

## style[0]

```css

:root { view-transition-name: none; }
.target {
  width: 100px;
  height: 100px;
  contain: paint;
  background: blue;
  overflow-clip-margin: 50px;
  view-transition-name: target;
}
.child {
  width: 200px;
  height: 200px;
  position: relative;
  top: 50px;
  left: 50px;
  background: green;
}

html::view-transition-group(target) { animation-duration: 300s; }
html::view-transition-new(target) { animation: unset; opacity: 0; }
html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}

/* None of these should apply, so make everything red if it does */
html::view-transition-group(root) { animation: unset; opacity: 1; background: red; }
html::view-transition-image-pair(root) { visibility: hidden }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
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
  "warnings": 1
}
```
