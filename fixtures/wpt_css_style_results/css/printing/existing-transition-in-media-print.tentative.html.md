# css/printing/existing-transition-in-media-print.tentative.html

```json
{
  "format_version": 3,
  "file": "css/printing/existing-transition-in-media-print.tentative.html"
}
```

## style[0]

```css

#target {
  transition-duration: 100000s;
  print-color-adjust: exact;
}
#target:not(.print) {
  color: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
