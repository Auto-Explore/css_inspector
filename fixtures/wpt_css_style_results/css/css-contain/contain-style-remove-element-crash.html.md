# css/css-contain/contain-style-remove-element-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-remove-element-crash.html"
}
```

## style[0]

```css

* {
  counter-reset: reversed(counter_3) reversed(counter_4) 64 reversed(counter_5) -8772;
  contain: layout paint style inline-size !important;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “counter-reset”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
