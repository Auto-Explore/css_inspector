# css/css-view-transitions/span-with-overflowing-text-hidden.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/span-with-overflowing-text-hidden.html"
}
```

## style[0]

```css

#target {
  view-transition-name: target;
  background: grey;
}
.hidden {
  view-transition-name: hidden;
}
span {
  text-shadow: red 2px 5px;
  visibility: hidden;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }
html::view-transition-group(root) { visibility: hidden; }

html::view-transition-group(target) {
  animation: unset;
}

html::view-transition-old(target) {
  animation: unset;
  opacity: 0;
}

html::view-transition-new(target) {
  animation: unset;
  opacity: 1;
}

html::view-transition {
  background: pink;
}
```

```json
{
  "errors": 12,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
