# css/css-view-transitions/pseudo-with-classes-match-multiple-wildcard.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-match-multiple-wildcard.html"
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
  view-transition-class: cls some-div;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  animation-play-state: paused;
}

::view-transition-new(*.cls.some-div),
::view-transition-old(*.cls.some-div) {
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
