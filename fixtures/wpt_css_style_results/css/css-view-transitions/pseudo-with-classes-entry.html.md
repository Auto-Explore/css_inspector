# css/css-view-transitions/pseudo-with-classes-entry.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-entry.html"
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

#target.new-state {
  view-transition-class: cls;
  view-transition-name: target;
}

@keyframes jump {
  from { opacity: 1;}
  to { opacity: 1;}
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-new(*) {
  animation-name: jump;
  animation-timing-function: step-start;
  animation-play-state: paused;
}

::view-transition-new(*.cls) {
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
