# css/css-transforms/crashtests/w-negative-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/crashtests/w-negative-001.html"
}
```

## style[0]

```css

* {
  transform: perspective(142176px);
  mix-blend-mode: darken;
  transform-origin: left center 537763.2px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
