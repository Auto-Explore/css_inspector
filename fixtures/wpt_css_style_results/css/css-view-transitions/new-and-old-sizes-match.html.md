# css/css-view-transitions/new-and-old-sizes-match.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-and-old-sizes-match.html"
}
```

## style[0]

```css

.box {
  width: 100px;
  height: 100px;
  contain: paint;
  position: absolute;
  overflow-clip-margin: 50px;
}
#target {
  top: 20px;
  left: 20px;
  view-transition-name: target;
}
.inner_overflow {
  width: 50px;
  height: 150px;
  margin-left: -10px;
  margin-top: -20px;
  background: lightgreen;
  clip-path: inset(1px 1px 1px 1px);
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
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
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
