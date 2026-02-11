# css/css-paint-api/background-repeat-x.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/background-repeat-x.https.html"
}
```

## style[0]

```css

.container {
  width: 200px;
  height: 200px;
}
#foo {
  background: paint(foo) top left/50% 50% repeat-x;
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
