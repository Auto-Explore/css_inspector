# css/css-view-transitions/iframe-and-main-frame-transition-new-main-old-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/iframe-and-main-frame-transition-new-main-old-iframe.html"
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

/* Keep showing the live state for the main frame so we can assert the state of
   screenshots in the iframe */
::view-transition-group(root) {
  animation-duration: 300s;
}
::view-transition-old(root) {
  animation: unset;
  opacity: 0;
}
::view-transition-new(root) {
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

  /* The iframe is showing old screenshots */
  ::view-transition-group(root) {
    animation-duration: 300s;
  }
  ::view-transition-old(root) {
    animation: unset;
    opacity: 1;
  }
  ::view-transition-new(root) {
    animation: unset;
    opacity: 0;
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
