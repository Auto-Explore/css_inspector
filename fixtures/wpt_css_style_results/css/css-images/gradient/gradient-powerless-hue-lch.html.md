# css/css-images/gradient/gradient-powerless-hue-lch.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-powerless-hue-lch.html"
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
        }

        .info {
            position: absolute;
            right: -10px;
            transform: translateX(100%);
        }

        .lch {
            /* Expected */
            background-image: linear-gradient(90deg in lch, red, lch(87.82 113.33 134.38 / 0) );
        }
        .hwb {
            background-image: linear-gradient(to right in lch, red, hwb(120deg 0% 0% / 0%) );
        }
        .rgba {
            background-image: linear-gradient(to right in lch, red, rgba(0, 255, 0, 0) );
        }
        .hsl {
            background-image: linear-gradient(to right in lch, red, hsl(120deg 100% 50% / 0%) );
        }
        .color {
            background-image: linear-gradient(90deg in lch, red, color(srgb 0 1 0 / 0) );
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
