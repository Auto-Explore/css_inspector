# css/css-shapes/spec-examples/reference/shape-outside-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/reference/shape-outside-007-ref.html"
}
```

## style[0]

```css

        #container {
          position: absolute;
          top: 70px;
          width: 260px;
          height: 120px;
          font-family: Ahem;
          font-size: 20px;
          line-height: 1.5em;
          border: 1px solid black;
          padding-left: 2px;
        }
        #margin-line {
          position: absolute;
          top: 100px;
          width: 263px;
          border-bottom: 1px solid black;
        }
        .bar {
            position: absolute;
            height: 20px;
            background-color: green;
        }
        #bar-1 {
            top: 5px;
            width: 240px;
        }
        #bar-2 {
            top: 35px;
            width: 240px;
        }
        #bar-3 {
            top: 65px;
            left: 22px;
            width: 220px;
        }
        #bar-4 {
            top: 95px;
            left: 52px;
            width: 200px;
        }
        #triangle {
            position: absolute;
            top: 90px;
            width: 100px;
            height: 100px;
            background-color: lightblue;
            margin-left: 2px;
            clip-path: polygon(0% 50%, 50% 100%, 0 100%);;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
