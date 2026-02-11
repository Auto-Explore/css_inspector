# css/css-transitions/animations/z-index-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/animations/z-index-interpolation.html"
}
```

## style[0]

```css

body {
  margin-top: 20px;
}
.layer-reference {
  position: fixed;
  top: 0px;
  height: 100vh;
  width: 50px;
  background-color: rgba(255, 255, 255, 0.75);
  font-family: sans-serif;
  text-align: center;
  padding-top: 5px;
  border: 1px solid;
}
.parent {
  z-index: 15;
}
.target {
  position: relative;
  width: 350px;
  height: 10px;
  z-index: -2;
}
.actual {
  background-color: black;
}
.expected {
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
