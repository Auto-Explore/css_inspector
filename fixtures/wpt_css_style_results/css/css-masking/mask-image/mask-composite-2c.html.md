# css/css-masking/mask-image/mask-composite-2c.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-composite-2c.html"
}
```

## style[0]

```css

      div {
        background-color: blue;
        position: absolute;
        margin: 0px;
        padding: 0px;
        width: 100px;
        height: 100px;
        top:10px;
        mask-image: url(#rectMask1),
                    url(#rectMask2);
      }

      div.add {
        left: 10px;
        mask-composite: add;
      }

      div.subtract {
        left: 120px;
        mask-composite: subtract;
      }

      div.intersect {
        left: 230px;
        mask-composite: intersect;
      }

      div.exclude {
        left: 340px;
        mask-composite: exclude;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
