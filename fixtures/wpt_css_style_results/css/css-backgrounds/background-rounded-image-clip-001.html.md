# css/css-backgrounds/background-rounded-image-clip-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-rounded-image-clip-001.html"
}
```

## style[0]

```css

    html {
        background-color: green;
    }
    #a {
        top: 20px;
        left: 20px;
        position: absolute;
        width: 20px;
        height: 20px;
        background-color: black;
    }

    #b {
        position: absolute;
        width: 300px;
        height: 200px;
        background-image: linear-gradient(green, green);
        background-clip: content-box;
        border-top-left-radius: 90px;
        border-width: 10px;
        border-style: solid;
        border-color: transparent;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
