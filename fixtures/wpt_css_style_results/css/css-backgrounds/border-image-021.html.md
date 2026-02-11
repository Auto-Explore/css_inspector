# css/css-backgrounds/border-image-021.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-021.html"
}
```

## style[0]

```css

  #target {
    width: 100px;
    height: 100px;
    background: conic-gradient(rgba(255, 0, 0, 0.5) 0 0), conic-gradient(red 0 0);
    border-radius: 40px;
    border-image: conic-gradient(green 0 0) 1 fill / 10px;
  }
  #back {
    width: 100px;
    height: 100px;
    background-color: red;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
