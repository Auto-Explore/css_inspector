# css/css-backgrounds/background-margin-will-change-root.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-margin-will-change-root.html"
}
```

## style[0]

```css

  html {
    background: linear-gradient(lightblue, yellow);
    height: 300px;
    margin: 50px;
    will-change: transform;
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
