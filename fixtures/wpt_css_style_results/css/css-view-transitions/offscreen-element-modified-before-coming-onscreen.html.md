# css/css-view-transitions/offscreen-element-modified-before-coming-onscreen.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/offscreen-element-modified-before-coming-onscreen.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  position: fixed;
  background: green;
  left: 0;
  top: 200vh;
}

.hidden {
  width: 10px;
  height: 10px;
  background: grey;
  view-transition-name: hidden;
}

.onscreen::view-transition-group(target) {
  transform: unset;
  position: fixed;
  top: 0;
  left: 0;
  animation: unset;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }

html::view-transition-old(target) { animation: unset; opacity: 0; }
html::view-transition-new(target) { animation: unset; opacity: 1; }

```

```json
{
  "errors": 8,
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
