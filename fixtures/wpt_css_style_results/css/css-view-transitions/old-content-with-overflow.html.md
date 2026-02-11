# css/css-view-transitions/old-content-with-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/old-content-with-overflow.html"
}
```

## style[0]

```css

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

html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: lightpink; }
```

```json
{
  "errors": 7,
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
    }
  ],
  "warnings": 1
}
```
