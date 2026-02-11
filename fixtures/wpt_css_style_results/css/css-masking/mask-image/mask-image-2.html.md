# css/css-masking/mask-image/mask-image-2.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-2.html"
}
```

## style[0]

```css

      div {
        background-color: purple;
        width: 100px;
        height: 100px;
      }

      div.mask-by-gradient-1 {
        mask-image: linear-gradient(rgba(0,0,255,0), rgba(0,0,255,1)); /* blue gradient mask */
      }

      div.mask-by-gradient-2 {
        mask-image: linear-gradient(rgba(255,0,0,0), rgba(255,0,0,1)); /* red gradient mask */
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
