# css/css-masking/mask-image/mask-opacity-1e.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-opacity-1e.html"
}
```

## style[0]

```css

      #outter {
        position: absolute;
        left: 10px;
        top: 10px;
        width: 100px;
        height: 100px;
        mask-image: url(support/blue-100x50-transparent-100x50.png),
                    url(support/blue-100x50-transparent-100x50.png);
        opacity: 0.5;
      }

      #inner {
        width: 100px;
        height: 100px;
        box-sizing:border-box;
        background-color: blue;
        border: 1px solid transparent;
        will-change: transform;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
