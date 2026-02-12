# css/css-masking/mask-image/mask-image-3e.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-3e.html"
}
```

## style[0]

```css

      div.outter {
        margin: 0px;
        padding: 0px;
        width: 200px;
        height: 200px;
        transform: translate(-10px, -10px);
      }
      div.inner {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg), url(#mask1);
        mask-repeat: no-repeat, repeat;
        mask-position: 0 0, 0 0;
        /*mask: url(#mask1);*/
        width: 100px;
        height: 100px;
        transform: translate(10px, 10px);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
