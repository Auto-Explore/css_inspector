# css/css-view-transitions/small-scale.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/small-scale.html"
}
```

## style[0]

```css

  :root {
    view-transition-name: none;
  }
  body { margin: 0 }
  .outer {
    /* Removing this makes it work */
    transform: scale(0.5);
  }
  .inner {
    view-transition-name: inner;
    width: 200px;
    height: 200px;
    background-color: green;
  }
  :root::view-transition {
    background: pink;
  }
  :root::view-transition-group(inner),
  :root::view-transition-image-pair(inner),
  :root::view-transition-old(inner),
  :root::view-transition-new(inner) {
    animation-play-state: paused;
  }
```

```json
{
  "errors": 5,
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
    }
  ],
  "warnings": 0
}
```
