# css/css-view-transitions/pseudo-element-overflow-clip-with-border-radius.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-element-overflow-clip-with-border-radius.html"
}
```

## style[0]

```css

body {
  margin: 0px;
}
div {
  width: 200px;
  height: 200px;
}
#target {
  width: 200px;
  height: 200px;
  background: green;
  view-transition-name: target;
}
#inner {
  position: relative;
  left: 100px;
  top: 100px;
  background: blue;
}

html::view-transition-group(*) { animation-duration: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 1; }
html::view-transition-old(*) { animation: unset; opacity: 1; }
html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: pink; }

html::view-transition-new(target) {
  overflow:clip;
  overflow-clip-margin: 30px;
  border-radius: 30px;
}
html::view-transition-old(target) {
  left: 400px;
  overflow: clip;
  overflow-clip-margin: 30px;
  border-radius: 30px;
}
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    }
  ],
  "warnings": 2
}
```
