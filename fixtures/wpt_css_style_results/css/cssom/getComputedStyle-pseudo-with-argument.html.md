# css/cssom/getComputedStyle-pseudo-with-argument.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-pseudo-with-argument.html"
}
```

## style[0]

```css

#pseudo-invalid::highlight(name) {
  color: rgb(0, 128, 0);
}
#pseudo-invalid::view-transition-image-pair(name) {
  color: rgb(0, 128, 0);
}
#pseudo-invalid::view-transition-group(name) {
  color: rgb(0, 128, 0);
}
#pseudo-invalid::view-transition-old(name) {
  color: rgb(0, 128, 0);
}
#pseudo-invalid::view-transition-new(name) {
  color: rgb(0, 128, 0);
}
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
