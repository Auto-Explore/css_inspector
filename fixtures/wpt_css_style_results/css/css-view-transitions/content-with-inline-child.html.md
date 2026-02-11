# css/css-view-transitions/content-with-inline-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-with-inline-child.html"
}
```

## style[0]

```css


#target {
  width: 100px;
  height: 100px;
  overflow-clip-margin: 500px;
  contain: paint;
  view-transition-name: target;
  background-color: grey;
}

#child {
  position: relative;
  left: 100px;
  top: 100px;
  color: lightgreen;
  background-color: darkgreen;
}

#innerchild {
  position: relative;
  left: 100px;
}

html::view-transition-new(root) {
  opacity: 0;
}
html::view-transition-old(root) {
  opacity: 0;
}

html::view-transition-old(target) {
  animation-duration: 3s;
  animation-timing-function: steps(1, end);
  opacity: 1;
}
html::view-transition-new(target) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
