# css/css-images/gradient/gradient-decreasing-hue-lch.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-decreasing-hue-lch.html"
}
```

## style[0]

```css

        body {
            background: #fff;
        }

        div {
            width: 200px;
            height: 100px;
            margin: 10px;
        }

        div.a {
            background-image: linear-gradient(to right in lch decreasing hue,
                                              lch(50% 100% 0deg),
                                              lch(50% 100% 80deg));
        }

        div.b {
            background-image: linear-gradient(to right in lch decreasing hue,
                                              lch(50% 100% 80deg),
                                              lch(50% 100% 0deg));
        }

        div.c {
            background-image: linear-gradient(to right in lch decreasing hue,
                                              lch(50% 100% 0deg),
                                              lch(50% 100% 270deg));
        }

        div.d {
            background-image: linear-gradient(to right in lch decreasing hue,
                                              lch(50% 100% 270deg),
                                              lch(50% 100% 0deg));
        }
    
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
