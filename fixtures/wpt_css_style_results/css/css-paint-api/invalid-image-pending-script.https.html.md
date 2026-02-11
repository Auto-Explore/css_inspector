# css/css-paint-api/invalid-image-pending-script.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/invalid-image-pending-script.https.html"
}
```

## style[0]

```css

    #output {
        width: 100px;
        height: 100px;
        background-image: paint(invalid), paint(successIndicator);
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
