# css/css-view-transitions/finished-promise-defers-cleanup.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/finished-promise-defers-cleanup.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
}
::view-transition-group(*) {
  animation-duration: 0ms;
}
::view-transition-old(target) {
  background-color: rgb(0, 255, 0);
}
```

```json
{
  "errors": 3,
  "messages": [
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
