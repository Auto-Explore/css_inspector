# css/css-images/gradient/gradient-powerless-hue-hwb-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-powerless-hue-hwb-ref.html"
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
            background-image: linear-gradient(to right in hwb, red, hwb(120deg 0% 0% / 0%) );
        }

        .info {
            position: absolute;
            right: -10px;
            transform: translateX(100%);
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
