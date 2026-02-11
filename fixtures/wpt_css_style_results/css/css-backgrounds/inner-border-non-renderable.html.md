# css/css-backgrounds/inner-border-non-renderable.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/inner-border-non-renderable.html"
}
```

## style[0]

```css

        body {
            font-size: 24px;
            color: black;
            margin: 8px;
        }
        .clipping {
            width: 300px;
            height: 200px;
            overflow: hidden;
            border: 30px solid green;
            border-top-color: gold;
            border-top-right-radius: 150px 267px;
            background-color: blue;
        }
        .composited {
            width: 100%;
            height: 100%;
            background-color: blue;
        }
        .clip-test {
            clip-path: inset(60px 10px 190px 320px);
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-*-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-right-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
