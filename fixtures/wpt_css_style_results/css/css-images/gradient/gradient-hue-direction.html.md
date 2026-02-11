# css/css-images/gradient/gradient-hue-direction.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-hue-direction.html"
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

        div.a1 {
            background-image: linear-gradient(to right in hsl increasing hue, red, orange);
        }
        div.a2 {
            background-image: linear-gradient(to right in hsl shorter hue, red, orange);
        }

        div.b1 {
            background-image: linear-gradient(to right in hsl decreasing hue, red, orange);
        }
        div.b2 {
            background-image: linear-gradient(to right in hsl longer hue, red, orange);
        }

        div.c1 {
            background-image: linear-gradient(to right in lch increasing hue, red, orange);
        }
        div.c2 {
            background-image: linear-gradient(to right in lch shorter hue, red, orange);
        }

        div.d1 {
            background-image: linear-gradient(to right in lch decreasing hue, red, orange);
        }
        div.d2 {
            background-image: linear-gradient(to right in lch longer hue, red, orange);
        }
    
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
