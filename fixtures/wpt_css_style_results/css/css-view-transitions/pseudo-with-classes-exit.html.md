# css/css-view-transitions/pseudo-with-classes-exit.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-exit.html"
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
}

#target:not(.new-state) {
  view-transition-class: cls;
  view-transition-name: target;
}

@keyframes jump {
  from { opacity: 1;}
  to { opacity: 1;}
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*) {
  animation-name: jump;
  animation-timing-function: step-start;
  animation-play-state: paused;
}

::view-transition-old(*.cls) {
  left: 100px;
}

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
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
