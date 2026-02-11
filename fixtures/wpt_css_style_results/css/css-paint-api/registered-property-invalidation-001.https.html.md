# css/css-paint-api/registered-property-invalidation-001.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/registered-property-invalidation-001.https.html"
}
```

## style[0]

```css

#target {
  background: paint(geometry);
  width: 100px;
  height: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
