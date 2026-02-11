# css/css-view-transitions/old-content-captures-root.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/old-content-captures-root.html"
}
```

## style[0]

```css

.box {
  background: lightblue;
  width: 100px;
  height: 100px;
  contain: paint;
  position: absolute;
  font-size: 30pt;
}
#e1 {
  top: 20px;
  left: 20px;
}
#shared {
  contain: paint;
  width: 100px;
  height: 100px;
  background: red;
  view-transition-name: shared;
}

div.dst { background: lightgreen; }
/* We're verifying what we capture, so just display the old contents for 5 minutes.  */
html::view-transition { background: pink; }
html::view-transition-group(shared) { animation-duration: 300s; }
html::view-transition-image-pair(shared) { visibility: hidden }
html::view-transition-old(root) { animation: unset; opacity: 1 }
html::view-transition-new(root) { animation: unset; opacity: 0 }
```

```json
{
  "errors": 9,
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
