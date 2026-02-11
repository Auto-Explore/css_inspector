# css/css-masking/mask-image/mask-image-3f.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-3f.html"
}
```

## style[0]

```css

      div {
        background-color: red;
        mask-image: url(#mask1), url(#mask2);
        box-shadow: 0 0 0 100px purple;
        width: 100px;
        height: 100px;
        position: relative;
        left: 100px;
        top: 100px;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
