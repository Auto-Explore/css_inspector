# css/css-view-transitions/class-specificity.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/class-specificity.html"
}
```

## style[0]

```css

#shared {
  contain: paint;
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: shared;
  view-transition-class: sharedclass1 sharedclass2;
}

/* We're verifying what we capture, so just display the old contents for 5 minutes.  */
html::view-transition { background: pink; }
html::view-transition-group(shared) { animation-duration: 300s; }

html::view-transition-old(shared) {
  animation: unset;
  opacity: 1;
  border: 10px solid red;
}
html::view-transition-old(*.sharedclass1.sharedclass2) {
  border: 10px solid yellow;
}

html::view-transition-old(*.sharedclass1) {
  border: 10px solid green;
}

html::view-transition-old(*) {
  border: 10px solid orange;
}

html::view-transition-new(shared) { animation: unset; opacity: 0; }

html::view-transition-old(root) { animation: unset; opacity: 0; }
html::view-transition-new(root) { animation: unset; opacity: 0 }
```

```json
{
  "errors": 13,
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
      "message": "Invalid value for property “background”.",
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
      "message": "Invalid value for property “border”.",
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
