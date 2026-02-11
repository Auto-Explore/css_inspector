# css/css-view-transitions/nested/group-children-animations.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/group-children-animations.html"
}
```

## style[0]

```css

:root {
  view-transition-name: none;
}
#container {
  view-transition-name: container;
  view-transition-group: contain;

  width: 20px;
  height: 20px;

  border: 12px solid black;
  border-radius: 20px;
  corner-shape: bevel;
}
#container.updated {
  border: 30px solid black;
  border-radius: 40px;
  corner-shape: squircle;
}

#child {
  view-transition-name: child;
  width: 10px;
  height: 10px;
}

::view-transition-group-children(*) {
  animation-duration: 5s;
  animation-play-state: paused;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  animation: unset;
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
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-group”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “corner-shape”.",
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
    }
  ],
  "warnings": 0
}
```
