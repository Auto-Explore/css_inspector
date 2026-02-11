# css/css-images/infinite-radial-gradient-refcrash.html

```json
{
  "format_version": 3,
  "file": "css/css-images/infinite-radial-gradient-refcrash.html"
}
```

## style[0]

```css

  #crash {
    background-image: repeating-radial-gradient(closest-corner circle at 9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999%, green, green);
    width: 300px;
    height: 300px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
