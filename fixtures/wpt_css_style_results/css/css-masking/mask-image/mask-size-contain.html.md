# css/css-masking/mask-image/mask-size-contain.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-contain.html"
}
```

## style[0]

```css

      div {
        width: 64px;
        height: 128px;
      }

      #outer {
        border: 10px solid black;
      }

      #inner {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-size: contain;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
