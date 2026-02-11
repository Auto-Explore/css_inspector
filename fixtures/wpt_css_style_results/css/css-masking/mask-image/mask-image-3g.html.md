# css/css-masking/mask-image/mask-image-3g.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-3g.html"
}
```

## style[0]

```css

      div.outter {
        margin: 0px;
        padding: 0px;
        width: 200px;
        height: 200px;
        transform: translate(-10px, -20px);
      }
      div.inner {
        background-color: red;
        mask-image: url(#mask1), url(#mask2);
        mask-repeat: no-repeat, no-repeat;
        box-shadow: 0 0 0 100px purple;
        position: relative;
        width: 100px;
        height: 100px;
        left: 100px;
        top: 100px;
        transform: translate(10px, 20px);
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
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
