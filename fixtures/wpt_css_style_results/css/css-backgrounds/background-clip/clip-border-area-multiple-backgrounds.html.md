# css/css-backgrounds/background-clip/clip-border-area-multiple-backgrounds.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-multiple-backgrounds.html"
}
```

## style[0]

```css

    .test {
        display: inline-block;
        margin: 20px;
        width: 300px;
        height: 200px;
        box-sizing: border-box;
        border: 50px dotted transparent;
        background-clip: border-area, border-box, content-box;
        background-color: red;
        background-image: url(../resources/blue-100.png), url(../resources/green-100.png), none;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
