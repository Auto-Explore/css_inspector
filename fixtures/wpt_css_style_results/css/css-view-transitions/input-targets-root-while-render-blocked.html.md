# css/css-view-transitions/input-targets-root-while-render-blocked.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/input-targets-root-while-render-blocked.html"
}
```

## style[0]

```css

:root {
  /* Ensure clicks during the transition fall through the pseudo tree root to
   * real DOM */
  view-transition-name: none;
}

::view-transition {
  /* Ensure clicks during the transition fall through the pseudo tree root to
   * real DOM */
  pointer-events: none;
  width: 0;
  height: 0;
}

::view-transition-group(*) {
  animation-duration: 30s;
}

#clicktarget {
  width: 100px;
  height: 100px;
  background: red;
}

#transition {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: transitionElement;
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “pointer-events”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
