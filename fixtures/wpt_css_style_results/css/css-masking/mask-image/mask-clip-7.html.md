# css/css-masking/mask-image/mask-clip-7.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-7.html"
}
```

## style[0]

```css

      div.mask {
        mask-image: linear-gradient(black, black);
        mask-clip: no-clip;
        background-color: purple;
        width: 40px;
        height: 20px;
        border-radius: 5px;
        position: relative;
      }
      span.mask {
        font: 20px/1 Ahem;
        line-height: 20px;
        mask-image: linear-gradient(black, black);
        mask-clip: no-clip;
        color: purple;
        border-radius: 5px;
        position: relative;
      }
      .unclipped-child {
        position: absolute;
        top: -6px;
        left: -6px;
        width: 52px;
        height: 32px;
        background: purple;
      }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
