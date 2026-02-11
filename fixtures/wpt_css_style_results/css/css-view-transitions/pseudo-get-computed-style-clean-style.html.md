# css/css-view-transitions/pseudo-get-computed-style-clean-style.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-get-computed-style-clean-style.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  mix-blend-mode: multiply;
}
::view-transition-image-pair(target) {
  position: fixed;
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
