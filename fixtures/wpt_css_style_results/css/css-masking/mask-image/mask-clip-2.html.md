# css/css-masking/mask-image/mask-clip-2.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-2.html"
}
```

## style[0]

```css

      svg {
        position: absolute;
        top: 10px;
        border: 1px solid black;
      }

      rect.mask {
        fill: blue;
        mask-origin: fill-box;
        mask-repeat: no-repeat;
        mask-image: url(support/50x50-opaque-blue.svg);
      }

      rect.view {
        mask-clip: view-box;
      }

      rect.fill {
        mask-clip: fill-box;
      }

      rect.stroke {
        mask-clip: stroke-box;
      }
    
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
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
