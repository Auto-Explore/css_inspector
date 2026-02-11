# css/css-overflow/line-clamp/webkit-line-clamp-with-line-height.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-with-line-height.html"
}
```

## style[0]

```css

#test {
  width: 100px;

  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 1;
  overflow: hidden;

  border: solid thin grey;
  font: 20px 'Ahem';
  line-height: 40px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
