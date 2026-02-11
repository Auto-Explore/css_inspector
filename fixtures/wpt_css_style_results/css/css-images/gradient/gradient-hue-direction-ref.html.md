# css/css-images/gradient/gradient-hue-direction-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-hue-direction-ref.html"
}
```

## style[0]

```css

        body {
            background: #fff;
        }

        div {
            width: 200px;
            height: 50px;
            margin: 10px;
        }

        div.a {
            background-image: linear-gradient(to right in hsl shorter hue, red, orange);
        }

        div.b {
            background-image: linear-gradient(to right in hsl longer hue, red, orange);
        }

        div.c {
            background-image: linear-gradient(to right in lch shorter hue, red, orange);
        }

        div.d {
            background-image: linear-gradient(to right in lch longer hue, red, orange);
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
