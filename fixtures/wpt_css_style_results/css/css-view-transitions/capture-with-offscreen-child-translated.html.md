# css/css-view-transitions/capture-with-offscreen-child-translated.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/capture-with-offscreen-child-translated.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  background: blue;
}
.invisible {
  position: absolute;
  top: -100px;
  width: 50px;
  height: 50px;
  background: green;
}

::view-transition-group(target) {
  animation: unset;
  top: 200px;
}

::view-transition-group(root) {
  visibility: hidden;
  animation-duration: 500s;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}

::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}

::view-transition {
  background: pink;
}
```

```json
{
  "errors": 7,
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
    }
  ],
  "warnings": 0
}
```
