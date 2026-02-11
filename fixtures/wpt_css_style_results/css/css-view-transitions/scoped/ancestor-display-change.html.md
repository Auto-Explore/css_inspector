# css/css-view-transitions/scoped/ancestor-display-change.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/ancestor-display-change.html"
}
```

## style[0]

```css

  .block {
    background-color: blue;
    margin: 10px;
    height: 100px;
    width: 100px;
  }
  #target {
    view-transition-name: target;
  }
  .fade-out {
    opacity: 0;
  }
  .flex {
    display: flex;
  }
  ::view-transition-group(*),
  ::view-transition-image-pair(*),
  ::view-transition-old(*),
  ::view-transition-new(*) {
    /* freeze all animations at mid-point */
    animation-duration: 100000s;
    animation-delay: -50000s;
    animation-timing-function: cubic-bezier(0, 1, 1, 0);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
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
