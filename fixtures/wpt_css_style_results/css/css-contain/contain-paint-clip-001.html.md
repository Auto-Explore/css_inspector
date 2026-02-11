# css/css-contain/contain-paint-clip-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-clip-001.html"
}
```

## style[0]

```css

  .root {
    contain: paint;
    width: 100px;
    height: 100px;
    background: blue;
    margin: 25px;
    padding: 25px;
  }
  .a {
    width: 100px;
    height: 200px;
    background: red;
  }
  .b {
    width: 150px;
    height: 150px;
    background: green;
    position: relative;
    top: -25px;
    left: -25px;
  }
  .background {
    position: absolute;
    top: 0;
    left: 0;
    width: 200px;
    height: 200px;
    background: red;
    z-index: -1;
  }
  .foreground {
    position: absolute;
    top: -25px;
    left: -25px;
    width: 150px;
    height: 150px;
    border: 25px solid red;
    z-index: 1;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
