# css/css-backgrounds/background-clip/clip-border-area-border-on-top-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-border-on-top-ref.html"
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
        background-image: url(../resources/green-100.png);
    }

    .test > div {
        width: 100%;
        height: 100%;
        background-color: orange;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
