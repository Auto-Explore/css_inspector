# css/css-view-transitions/old-content-intrinsic-aspect-ratio.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/old-content-intrinsic-aspect-ratio.html"
}
```

## style[0]

```css

.spacer {
  height: 10px;
}
.box {
  width: 100px;
  height: 100px;
  contain: paint;
  background: blue;
}
#target1 {
  view-transition-name: target1;
}
#target2 {
  view-transition-name: target2;
}
#hidden {
  view-transition-name: hidden;
  width: 10px;
  height: 10px;
  visibility: hidden;
  contain: paint;
}

/* We're verifying what we capture, so just display the new contents for 5 minutes.  */
html::view-transition-group(*) { animation-duration: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 0; }
html::view-transition-old(*) { animation: unset; opacity: 1; }

html::view-transition-group(target1) {
  animation: unset;
  width: 50px;
  border: 2px solid black;
}
html::view-transition-group(target2) {
  animation: unset;
  width: 200px;
  border: 2px solid black;
}

/* hide the root so we show transition background to ensure we're in a transition */
html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: lightpink; }
```

```json
{
  "errors": 11,
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
  "warnings": 0
}
```
