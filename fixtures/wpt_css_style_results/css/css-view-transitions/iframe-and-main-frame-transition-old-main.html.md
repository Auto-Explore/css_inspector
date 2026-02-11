# css/css-view-transitions/iframe-and-main-frame-transition-old-main.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/iframe-and-main-frame-transition-old-main.html"
}
```

## style[0]

```css

iframe {
  position: fixed;
  top: 0;
  left: 0;
  width: 50vw;
  height: 50vh;
}

/* The main frame is showing the old screenshot */
::view-transition-group(root) {
  animation-duration: 300s;
}
::view-transition-new(root) {
  animation: unset;
  opacity: 0;
}
::view-transition-old(root) {
  animation: unset;
  opacity: 1;
}
```

```json
{
  "errors": 3,
  "messages": [
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

## style[1]

```css

  body {
    background: orange;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
