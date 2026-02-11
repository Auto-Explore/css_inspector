# css/css-overflow/line-clamp/webkit-line-clamp-bug-1926732-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-bug-1926732-crash.html"
}
```

## style[0]

```css

#a {
  display: -webkit-inline-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
