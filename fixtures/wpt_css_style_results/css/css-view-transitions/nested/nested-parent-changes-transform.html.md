# css/css-view-transitions/nested/nested-parent-changes-transform.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/nested-parent-changes-transform.html"
}
```

## style[0]

```css

:root { view-transition-name: none; }

.container {
  width: 100px;
  height: 100px;
  position: relative;
  background: red;

  view-transition-name: container;
  view-transition-group: contain;
}

.container.shifted {
  top: 150px;
}

.contained {
  width: 100px;
  height: 100px;
  background: green;

  view-transition-name: contained;
}

.hidden {
  display: none;
}

::view-transition-group(*) {
  animation-play-state: paused;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}

::view-transition-new(*) {
  animation: unset;
  opacity: 0;
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
