# css/css-masking/mask-image/mask-position-1b.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-1b.html"
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

      #inner1 { mask-position: left 40px top 15px; }
      #inner2 { mask-position: top 30% left 80%; }
      #inner3 { mask-position: left 20px top 25px }
      #inner4 { mask-position: top 25px left 20px; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
