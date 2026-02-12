# css/css-masking/mask-image/mask-repeat-1.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-repeat-1.html"
}
```

## style[0]

```css

      div {
        width: 115px;
        height: 115px;
      }

      div.outer {
        border: 1px solid black;
      }

      div.inner {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-position: left top;
      }

      #no-repeat {
        mask-repeat: no-repeat no-repeat;
      }
      #repeat {
        mask-repeat: repeat repeat;
      }
      #repeat-x {
        mask-repeat: repeat no-repeat;
      }
      #repeat-y {
        mask-repeat: no-repeat repeat;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
