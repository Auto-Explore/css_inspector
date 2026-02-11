# css/css-masking/mask-image/firefox-bug-1928736-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/firefox-bug-1928736-crash.html"
}
```

## style[0]

```css

input:in-range {
  mask-image: url(#a);
}
* {
  zoom: 32%;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
