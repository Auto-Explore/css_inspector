# css/css-view-transitions/pseudo-get-computed-style.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-get-computed-style.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: target;
  mix-blend-mode: multiply;
  text-orientation: upright;
  color-scheme: dark light;
}
::view-transition-image-pair(target) {
  position: fixed;
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
      "message": "Invalid value for property “color-scheme”.",
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
