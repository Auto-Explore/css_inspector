# css/css-backgrounds/background-clip/clip-border-area-border-on-top.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-border-on-top.html"
}
```

## style[0]

```css

    .test {
        margin: 20px;
        width: 300px;
        height: 200px;
        box-sizing: border-box;
        border: 50px solid rgba(0, 128, 0, 0.75);
        background-clip: border-area, border-box;
        background-color: orange;
        background-image: url(../resources/green-100.png), none;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-clip”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
