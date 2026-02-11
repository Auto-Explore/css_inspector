# css/css-anchor-position/anchor-transition-name.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-name.html"
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
  .anchor-name {
    anchor-name: --a;
  }
  #anchored {
    position: absolute;
    width: anchor-size(width);
    height: anchor-size(height);
    position-anchor: --a;
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
  "errors": 5,
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
      "message": "Unknown property “anchor-name”.",
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
