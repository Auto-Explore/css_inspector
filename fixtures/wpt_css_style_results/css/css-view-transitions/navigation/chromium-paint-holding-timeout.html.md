# css/css-view-transitions/navigation/chromium-paint-holding-timeout.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/chromium-paint-holding-timeout.html"
}
```

## style[0]

```css

@view-transition {
  navigation: auto;
}
.left {
  view-transition-name: target;
  width: 100px;
  height: 100px;
  background: red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
