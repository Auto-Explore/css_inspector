# css/css-images/gradient/gradient-powerless-hue-lch-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-powerless-hue-lch-ref.html"
}
```

## style[0]

```css

       .test {
            display: flex;
            align-items: center;
            width: 200px;
            height: 50px;
            position: relative;
            border: 1px solid black;
            margin: 10px;

            /* Expected */
            background-image: linear-gradient(90deg in lch, red, lch(87.82 113.33 134.38 / 0) );
        }

        .info {
            position: absolute;
            right: -10px;
            transform: translateX(100%);
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
