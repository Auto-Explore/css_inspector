# css/css-overflow/line-clamp/webkit-line-clamp-035.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-035.html"
}
```

## style[0]

```css

#parent {
  display: inline-block;
  background: green;
}
#clamp {
  font-family: Ahem;
  visibility: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  width: 4ch;
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
