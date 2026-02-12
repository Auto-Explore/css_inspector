# css/css-masking/mask-image/mask-origin-3.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-origin-3.html"
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
        mask-clip: fill-box;
        mask-repeat: no-repeat;
        mask-image: url(support/50x50-opaque-blue.svg);
      }

      rect.view {
        mask-origin: view-box;
        mask-clip: view-box;
      }

      rect.fill {
        mask-origin: fill-box;
        mask-clip: stroke-box;
      }

      rect.stroke {
        mask-origin: stroke-box;
        mask-clip: stroke-box;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
