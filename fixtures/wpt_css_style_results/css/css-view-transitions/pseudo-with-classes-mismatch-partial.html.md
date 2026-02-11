# css/css-view-transitions/pseudo-with-classes-mismatch-partial.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-mismatch-partial.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  position: absolute;
}

#target {
  background: green;
  view-transition-name: target;
  view-transition-class: cls;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  animation-play-state: paused;
}

::view-transition-new(*),
::view-transition-old(*) {
  left: 100px;
}
::view-transition-new(target.cls.other),
::view-transition-old(target.cls.other) {
  left: 0;
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
