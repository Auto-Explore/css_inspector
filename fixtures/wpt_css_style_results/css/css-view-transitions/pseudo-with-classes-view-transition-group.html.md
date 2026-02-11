# css/css-view-transitions/pseudo-with-classes-view-transition-group.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-view-transition-group.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  position: absolute;
  background: green;
  view-transition-name: target;
  view-transition-class: cls;
}

:root::view-transition-group(*),
:root::view-transition-image-pair(*),
:root::view-transition-new(*),
:root::view-transition-old(*) {
  animation-play-state: paused;
}

:root::view-transition-group(target.cls) {
  left: 100px;
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-class”.",
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
