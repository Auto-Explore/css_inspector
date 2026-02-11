# css/css-images/image-set/image-set-resolution-001.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-resolution-001.html"
}
```

## style[0]

```css

  body { margin: 0 }
  div {
    /* green.png is 100x50, should be 200x100 instead */
    content: -webkit-image-set(url('/images/green.png') 0.5x);
    content: image-set(url('/images/green.png') 0.5x);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
