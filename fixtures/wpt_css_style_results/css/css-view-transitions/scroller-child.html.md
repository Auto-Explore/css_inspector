# css/css-view-transitions/scroller-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scroller-child.html"
}
```

## style[0]

```css

#target {
  view-transition-name: target;
  width: 500px;
  height: 500px;
  background: yellow;
}
#scroller {
  overflow: scroll;
  width: 100px;
  height: 100px;
  background: blue;
}

#child {
  position: relative;
  width: 2000px;
  height: 2000px;
  background: green;
}

html::view-transition-group(root) { animation-duration: 300s; }
html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}
html::view-transition-new(target) {
  animation: unset;
  opacity: 0;
}
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
