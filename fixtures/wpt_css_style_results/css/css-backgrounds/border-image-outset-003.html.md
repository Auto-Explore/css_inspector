# css/css-backgrounds/border-image-outset-003.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-outset-003.html"
}
```

## style[0]

```css

    #a {
        width: 300px;
        height: 200px;
        position: absolute;
        left: 200px;
        top: 150px;
        border-width: 50px;
        border-style: solid;
        border-image: linear-gradient(green, green);
        background-color: blue;
        background-clip: content-box;
        border-image-outset: 50px 10px 50px 100px;
        border-image-slice: 33%;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image-outset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
