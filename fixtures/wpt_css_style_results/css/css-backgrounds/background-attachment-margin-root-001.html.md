# css/css-backgrounds/background-attachment-margin-root-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-attachment-margin-root-001.html"
}
```

## style[0]

```css

  html {
    background: linear-gradient(rgba(0,255,0,0.5), rgba(0,0,255,0.5)), linear-gradient(rgba(0,0,0,1), rgba(0,0,0,1));
    background-attachment: scroll, fixed;
    background-size: 100px 100px, 100px 100px;
    height: 300px;
    margin: 50px;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-attachment”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
