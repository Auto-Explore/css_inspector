# css/css-view-transitions/escaped-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/escaped-name.html"
}
```

## style[0]

```css

:root { view-transition-name: none; }
#first { view-transition-name: first\!; }
#second { view-transition-name: secon\'d; }
#third { view-transition-name: third\000021; }
.box {
  background: green;
  width: 100px;
  height: 100px;
  position: relative;
}
::view-transition-group(*),
::view-transition-new(*),
::view-transition-old(*),
::view-transition-image-pair(*) {
  animation-play-state: paused;
}
.box.after {
  background: red;
  left: 100px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
