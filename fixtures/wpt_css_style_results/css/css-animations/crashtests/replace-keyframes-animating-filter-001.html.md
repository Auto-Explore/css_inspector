# css/css-animations/crashtests/replace-keyframes-animating-filter-001.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/crashtests/replace-keyframes-animating-filter-001.html"
}
```

## style[0]

```css

#el1 {
  height: 100px;
  width: 100px;
  animation: kf1 2s linear;
}
@keyframes kf1 {
  from { filter: grayscale(0.9) }
  to { filter: grayscale(0.8) }
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
