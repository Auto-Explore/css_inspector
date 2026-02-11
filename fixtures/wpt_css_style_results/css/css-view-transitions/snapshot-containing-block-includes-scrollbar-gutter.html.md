# css/css-view-transitions/snapshot-containing-block-includes-scrollbar-gutter.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/snapshot-containing-block-includes-scrollbar-gutter.html"
}
```

## style[0]

```css

  :root {
    scrollbar-gutter: stable both-edges;
    /* unset so ::view-transition is visible. */
    view-transition-name: none;
  }
  ::view-transition {
    background-color: palegreen;
  }
  #target {
    position: absolute;
    top: 100px;
    left: 0px;
    width: 200px;
    height: 200px;
    background-color: limegreen;
    view-transition-name: target;
  }
  ::view-transition-group(target) {
    animation-duration: 300s;
  }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-gutter”.",
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
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
