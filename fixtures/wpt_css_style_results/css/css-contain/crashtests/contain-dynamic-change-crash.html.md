# css/css-contain/crashtests/contain-dynamic-change-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/crashtests/contain-dynamic-change-crash.html"
}
```

## style[0]

```css

*:only-child {
  contain: size layout paint;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
