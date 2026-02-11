# css/css-images/gradient/gradient-single-stop-005.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-single-stop-005.html"
}
```

## style[0]

```css

        body {
            background: #fff;
        }

        div {
            width: 100px;
            height: 100px;
            position: absolute;
        }

        #fail {
            background: red;
        }

        #test {
            background-image: repeating-radial-gradient(10px circle at top left, green);
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
