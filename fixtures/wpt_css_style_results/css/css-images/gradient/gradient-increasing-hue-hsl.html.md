# css/css-images/gradient/gradient-increasing-hue-hsl.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-increasing-hue-hsl.html"
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
            background-image: linear-gradient(to right in hsl increasing hue,
                                              hsl(0deg, 100%, 50%),
                                              hsl(40deg, 100%, 50%));
        }

        div.b {
            background-image: linear-gradient(to right in hsl increasing hue,
                                              hsl(40deg, 100%, 50%),
                                              hsl(0deg, 100%, 50%));
        }

        div.c {
            background-image: linear-gradient(to right in hsl increasing hue,
                                              hsl(0deg, 100%, 50%),
                                              hsl(270deg, 100%, 50%));
        }

        div.d {
            background-image: linear-gradient(to right in hsl increasing hue,
                                              hsl(270deg, 100%, 50%),
                                              hsl(0deg, 100%, 50%));
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
