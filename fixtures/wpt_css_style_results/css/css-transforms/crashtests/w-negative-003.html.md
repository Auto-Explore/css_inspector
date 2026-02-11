# css/css-transforms/crashtests/w-negative-003.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/crashtests/w-negative-003.html"
}
```

## style[0]

```css

button {
  transform:translate3d(1866px, 1794px, 1359px) perspective(848px);
  clip-path: ellipse(822px 405px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
