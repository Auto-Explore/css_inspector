# css/css-backgrounds/background-clip/clip-border-area-background-geometry-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-background-geometry-ref.html"
}
```

## style[0]

```css

    .test {
        margin: 20px;
        width: 300px;
        height: 200px;
        box-sizing: border-box;
        border: 50px solid transparent;
        background-image: url(../resources/stripes-100.png);
        background-size: 100px 100px;
        background-position: 15px 20px;
    }

    .test > div {
        width: 100%;
        height: 100%;
        background-color: white;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
