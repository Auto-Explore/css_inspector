# css/css-view-transitions/event-pseudo-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/event-pseudo-name.html"
}
```

## style[0]

```css

:root { view-transition-name: none; }
#first {
  background: blue;
  width: 100px;
  height: 100px;
  view-transition-name: shared;
}

html::view-transition-group(*),
html::view-transition-image-pair(*),
html::view-transition-new(*),
html::view-transition-old(*) {
  animation-duration: 600s;
}

@keyframes fade-in {
  from { opacity: 0; }
}
html::view-transition-image-pair(*) {
  animation: fade-in 600s both;
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
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
