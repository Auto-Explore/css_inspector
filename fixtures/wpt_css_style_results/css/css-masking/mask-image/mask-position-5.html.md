# css/css-masking/mask-image/mask-position-5.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-5.html"
}
```

## style[0]

```css

      div {
        width: 120px;
        height: 120px;
      }

      #outer {
        border: 1px solid black;
      }

      #inner {
        background-color: purple;
        mask-position: top, bottom;
        mask-repeat: no-repeat, no-repeat;
        mask-image: url(support/50x50-opaque-blue.svg),
                    url(support/50x50-opaque-blue.svg);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
