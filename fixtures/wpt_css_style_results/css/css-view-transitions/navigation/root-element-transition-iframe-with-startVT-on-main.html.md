# css/css-view-transitions/navigation/root-element-transition-iframe-with-startVT-on-main.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/root-element-transition-iframe-with-startVT-on-main.html"
}
```

## style[0]

```css

  #inner {
    view-transition-name: inner
  }

  ::view-transition {
    background: lightpink;
  }
  ::view-transition-group(root) {
    visibility: hidden;
    animation-play-state: paused;
  }

  ::view-transition-old(inner), ::view-transition-new(inner) {
    animation: unset;
  }
  ::view-transition-old(inner) {
    opacity: 0;
  }
  ::view-transition-new(inner) {
    opacity: 1;
  }
  ::view-transition-group(inner) {
    animation-duration: 0s;
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
