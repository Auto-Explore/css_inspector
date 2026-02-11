# css/css-view-transitions/new-content-flat-transform-ancestor.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-flat-transform-ancestor.html"
}
```

## style[0]

```css

body {
  perspective: 1000px;
}
.box {
  background: lightblue;
  width: 100px;
  height: 100px;
  transform: rotateY(60deg);
  view-transition-name: target;
}
.outer {
  width: 100px;
  height: 100px;
  transform: rotateY(60deg);
  perspective: 1000px;
}

/* We're verifying what we capture, so just display the new contents for 5 minutes.  */
html::view-transition-group(*) { animation-duration: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 1; }
html::view-transition-old(*) { animation: unset; opacity: 0; }
/* hide the root so we show transition background to ensure we're in a transition */
html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: lightpink; }
```

```json
{
  "errors": 8,
  "messages": [
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
