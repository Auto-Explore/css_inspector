# css/css-view-transitions/exit-transition-with-anonymous-layout-object.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/exit-transition-with-anonymous-layout-object.html"
}
```

## style[0]

```css

:root {
  scrollbar-width: none;
}

body {
  width: 100vw;
  height: 100vh;
}

#target {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
}

#hidden {
  width: 10px;
  height: 10px;
  background: grey;
  view-transition-name: hidden;
}

fieldset {
  width: 100px;
  height: 100px;
  background: lightgreen;
  overflow: clip;
}

html::view-transition { background: pink; }
html::view-transition-group(root) { visibility: hidden; }
html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }
html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
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
      "message": "Invalid value for property “background”.",
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
