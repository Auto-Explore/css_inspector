# css/css-view-transitions/nested/rounded-border-clipper.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/rounded-border-clipper.html"
}
```

## style[0]

```css

:root { view-transition-name: none }

#clipper {
  view-transition-group: contain;
  view-transition-name: clipper;
  overflow: clip;
  height: 200px;
  width: 100px;
  border-radius: 20px;
}

.item {
  will-change: transform;
  view-transition-name: item;
  background: green;
  position: relative;
  height: 50px;
}

.lower {
  top: 50px;
}

::view-transition-group(clipper) {
  animation-play-state: paused;
  overflow: clip;
  border-radius: 20px;
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
      "message": "Unknown property “view-transition-group”.",
      "severity": "Error"
    },
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
    }
  ],
  "warnings": 0
}
```
