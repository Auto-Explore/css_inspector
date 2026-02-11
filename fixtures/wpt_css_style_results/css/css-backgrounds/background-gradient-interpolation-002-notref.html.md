# css/css-backgrounds/background-gradient-interpolation-002-notref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-gradient-interpolation-002-notref.html"
}
```

## style[0]

```css

.has-gradient {
  background-image: linear-gradient(
    90deg,
    yellow 30%,
    purple 95%
  );
}

.text {
  font: 50px/1 Ahem;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  text-fill-color: transparent;
  width: fit-content;
  margin: 0;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-background-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-fill-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-fill-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
