# css/css-masking/mask-image/mask-repeat-3.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-repeat-3.html"
}
```

## style[0]

```css

      div {
        width: 150px;
        height: 150px;
      }

      div.outer {
        border: 1px solid black;
      }

      div.inner {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-position: left top;
      }

      #round {
        mask-repeat: round;
      }
      #round-x {
        mask-repeat: round no-repeat;
      }
      #round-y {
        mask-repeat: no-repeat round;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
