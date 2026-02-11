# css/css-view-transitions/modify-style-via-cssom.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/modify-style-via-cssom.html"
}
```

## style[0]

```css

#box {
  width: 100px;
  height: 100px;
  background: limegreen;
}
html::view-transition-group(root) {
  animation-duration: 300s;
}
html::view-transition-new(root) {
  animation: none;
  opacity: 0;
}
html::view-transition-old(root) {
  animation: none;
  opacity: 1;
}
```

```json
{
  "errors": 4,
  "messages": [
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
    }
  ],
  "warnings": 0
}
```
