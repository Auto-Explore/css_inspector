# css/css-masking/mask-image/mask-image-1c.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-1c.html"
}
```

## style[0]

```css

      div {
        background-color: purple;
        width: 100px;
        height: 100px;
      }

      div.mask-by-svg-mask {
        mask-image: url(support/mask-half-transparent-100x100.svg#mask);
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
