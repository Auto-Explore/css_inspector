# css/css-view-transitions/document-active-view-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/document-active-view-transition.html"
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
  animation-duration: 1ms;
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
