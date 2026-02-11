# css/css-anchor-position/anchor-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-animation.html"
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
  }
  @keyframes --anim {
    from { top: anchor(--a1 top); }
    to { top: anchor(--a2 bottom); }
  }
```

```json
{
  "errors": 7,
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
    }
  ],
  "warnings": 0
}
```
