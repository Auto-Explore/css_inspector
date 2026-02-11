# css/css-image-animation/image-animation-pseudo-animated-image.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-image-animation/image-animation-pseudo-animated-image.tentative.html"
}
```

## style[0]

```css

  img {
    display: block;
    margin: 10px;
    width: 20px;
    height: 20px;
  }

  img {
    outline: 5px solid red;
  }

  img:animated-image {
    image-animation: paused;
    outline: 5px solid green;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “image-animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
