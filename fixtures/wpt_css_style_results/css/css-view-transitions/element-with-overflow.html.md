# css/css-view-transitions/element-with-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/element-with-overflow.html"
}
```

## style[0]

```css

  .hidden {
    width: 10px;
    height: 10px;
    view-transition-name: hidden;
    background: green;
  }

  .target {
    width: 100px;
    height: 100px;
    background: lightblue;
    view-transition-name: target;
  }
  .inner {
    width: 100px;
    height: 100px;
    position: relative;
    top: 50px;
    left: 50px;
    border: 5px solid black;
  }

  html::view-transition-group(hidden) { animation-duration: 300s; }
  html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }

  html::view-transition-new(*), html::view-transition-old(*) {
    opacity: 1;
    animation: unset;
  }

  html::view-transition-old(target) {
    top: 0px;
    left: 0px;
  }

  html::view-transition-new(target) {
    top: 200px;
    left: 0px;
  }
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
