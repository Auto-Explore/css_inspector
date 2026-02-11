# css/css-view-transitions/pseudo-with-classes-multiple-vt-classes.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-multiple-vt-classes.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  position: absolute;
  view-transition-class: cls some-div;
}

#target {
  background: green;
  view-transition-name: target;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  animation-play-state: paused;
}

::view-transition-new(target.cls),
::view-transition-old(target.cls) {
  left: 100px;
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “view-transition-class”.",
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
