# css/css-anchor-position/anchor-transition-attr.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-attr.tentative.html"
}
```

## style[0]

```css

  #cb {
    display: inline-block;
    position: relative;
    width: 250px;
    height: 250px;
    border: 1px solid black;
  }
  #anchor1, #anchor2 {
    width: 100px;
  }
  #anchor1 {
    background: wheat;
    height: 50px;
  }
  #anchor2 {
    background: tomato;
    height: 90px;
  }
  #anchored {
    position: absolute;
    position-anchor: auto;
    width: anchor-size(width);
    height: anchor-size(height);
    top: anchor(top);
    left: anchor(right);
    transition-duration: 1000s;
    transition-timing-function: steps(2, start);
    transition-property: top, height;
    background-color: skyblue;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
