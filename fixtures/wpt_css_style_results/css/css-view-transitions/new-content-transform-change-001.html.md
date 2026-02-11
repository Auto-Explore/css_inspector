# css/css-view-transitions/new-content-transform-change-001.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-transform-change-001.html"
}
```

## style[0]

```css

  :root {
    view-transition-name: none;
  }

  #target {
    view-transition-name: target;
    width: 100px;
    height: 100px;
    background-color: green;
    transform: translate(0);
  }

  :root::view-transition {
    background-color: pink;
  }

  /* Just something that doesn't animate transform, so that the transform change takes effect immediately */
  @keyframes opacity-anim {
    from { opacity: 1 }
    to { opacity: 1 }
  }

  :root::view-transition-group(*) {
    animation-name: opacity-anim;
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
      "message": "Invalid value for property “background-color”.",
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
