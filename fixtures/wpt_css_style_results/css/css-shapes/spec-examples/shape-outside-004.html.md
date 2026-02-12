# css/css-shapes/spec-examples/shape-outside-004.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/shape-outside-004.html"
}
```

## style[0]

```css

        .container {
          width: 400px;
          font-family: Ahem;
          font-size: 20px;
          line-height: 1.5em;
        }
        #test {
            color: green;
        }
        #test-float-left {
          shape-outside: inset(50% 50% 50% 50%);
          float: left;
          width: 200px;
          height: 200px;
        }
        #failure {
            color: red;
            z-index: -1;
        }
        #test, #failure {
            position: absolute;
            top: 70px;
        }
        #square{
            position: absolute;
            top: 70px;
            width: 196px;
            height: 196px;
            border: 1px solid black;
            z-index: 10;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
