# css/css-view-transitions/nested/nested-position-with-border.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/nested-position-with-border.html"
}
```

## style[0]

```css

:root {
  view-transition-name: none;
}
#clipper {
  view-transition-group: contain;
  view-transition-name: clipper;

  border-width: 6px 10px 16px 20px;
  border-style: solid;
  border-color: red;

  height: 200px;
  width: 200px;
}

.item {
  view-transition-name: item;
  background: blue;
  position: relative;

  width: 100px;
  height: 100px;
  border: 1px solid black;
}

::view-transition {
  background: pink;
}
::view-transition-group-children(clipper) {
  overflow: clip;
  border-color: green;
}

::view-transition-group(*),
::view-transition-new(*),
::view-transition-old(*) {
  animation-play-state: paused;
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
    }
  ],
  "warnings": 0
}
```
