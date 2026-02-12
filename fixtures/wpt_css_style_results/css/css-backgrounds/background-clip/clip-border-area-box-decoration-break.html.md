# css/css-backgrounds/background-clip/clip-border-area-box-decoration-break.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-box-decoration-break.html"
}
```

## style[0]

```css

    div {
        margin: 20px;
        font-size: 24pt;
        line-height: 1.5em;
    }

    .test {
        border: 10px solid transparent;
        background-clip: border-area;
        background-image: url(../resources/blue-100.png);
    }

    .clone {
        -webkit-box-decoration-break: clone;
        box-decoration-break: clone;
    }

    .slice {
        -webkit-box-decoration-break: slice;
        box-decoration-break: slice;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
