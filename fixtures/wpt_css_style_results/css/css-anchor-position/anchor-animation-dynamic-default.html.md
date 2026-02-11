# css/css-anchor-position/anchor-animation-dynamic-default.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-animation-dynamic-default.html"
}
```

## style[0]

```css

  #cb {
    border: 1px solid black;
    width: 400px;
    height: 400px;
    position: relative;
  }
  #anchor1, #anchor2 {
    width: 100px;
    height: 100px;
    background: tomato;
  }
  #anchor1 {
    background: coral;
    anchor-name: --a1;
  }
  #anchor2 {
    background: seagreen;
    anchor-name: --a2;
  }
  #anchored {
    width: 50px;
    height: 50px;
    background: skyblue;
    animation: --anim 9999s steps(2, start);
    position: absolute;
    position-anchor: --a1;
  }
  @keyframes --anim {
    from { top: anchor(top); }
    to { top: anchor(bottom); }
  }
```

```json
{
  "errors": 8,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
