# css/css-masking/mask-image/mask-position-1c.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-1c.html"
}
```

## style[0]

```css

      div {
        width: 100px;
        height: 100px;
      }

      div.outer {
        border: 1px solid black;
      }

      div.inner {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
      }

      #inner1 { mask-position: left 80% bottom 70%; }
      #inner2 { mask-position: right 20% top 30%; }
      #inner3 { mask-position: bottom 50% left 40%; }
      #inner4 { mask-position: right 60% top 50%; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
