# css/css-masking/mask-image/mask-image-4b.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-4b.html"
}
```

## style[0]

```css

      div {
        background-color: purple;
        width: 100px;
        height: 100px;
      }

      div.mask-by-reference {
        mask-image: url(#clip1);
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
