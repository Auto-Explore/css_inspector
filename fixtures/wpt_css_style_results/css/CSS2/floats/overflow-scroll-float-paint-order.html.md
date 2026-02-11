# css/CSS2/floats/overflow-scroll-float-paint-order.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/overflow-scroll-float-paint-order.html"
}
```

## style[0]

```css

  #scroller {
    float: left;
    background: red;
    padding: 20px;
    box-sizing: border-box;
    width: 100px;
    height: 100px;
    overflow: scroll;
  }
  #negative-margin {
    float: left;
    width: 100px;
    height: 100px;
    background: green;
    margin-left: -100px;
  }
  #foreground1 {
    display: inline-block;
    width: 50px;
    height: 50px;
    background: blue;
  }
  #foreground2 {
    display: inline-block;
    width: 50px;
    height: 50px;
    background: magenta;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
