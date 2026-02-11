# css/CSS2/floats/crashtests/firefox-bug-1904409.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/crashtests/firefox-bug-1904409.html"
}
```

## style[0]

```css

* {
  float: inline-start;
  column-count: 15;
  text-emphasis-style: '@'
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “float”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
