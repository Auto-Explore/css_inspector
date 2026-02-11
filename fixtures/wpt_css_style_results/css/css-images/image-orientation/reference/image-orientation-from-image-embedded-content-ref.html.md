# css/css-images/image-orientation/reference/image-orientation-from-image-embedded-content-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-orientation/reference/image-orientation-from-image-embedded-content-ref.html"
}
```

## style[0]

```css

    iframe {
        display: inline-block;
        width: 120px;
        height: 110px;
        vertical-align: top;
    }
    embed {
        object-fit: none;
        object-position: top left;
        image-orientation: from-image;
    }
    object {
        object-fit: none;
        object-position: top left;
        image-orientation: from-image;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
