# css/css-masking/mask-image/mask-clip-4.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-4.html"
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

      rect.content {
        mask-clip: content-box; /* should be the same as fill-box */
      }

      rect.padding {
        mask-clip: padding-box; /* should be the same as fill-box */
      }

      rect.border {
        mask-clip: border-box; /* should be the same as stroke-box */
      }

      rect.margin {
        mask-clip: margin-box; /* should be the same as stroke-box */
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
