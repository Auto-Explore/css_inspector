# css/css-paint-api/setTransform-001.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/setTransform-001.https.html"
}
```

## style[0]

```css

.container {
  width: 200px;
  height: 200px;
}

#foo {
  background: paint(foo);
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
