# css/css-overflow/line-clamp/webkit-line-clamp-034.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-034.html"
}
```

## style[0]

```css

#clamp {
  visibility: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  overflow: hidden; /* can be removed once implementations update their old -webkit-line-clamp implementations */
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
