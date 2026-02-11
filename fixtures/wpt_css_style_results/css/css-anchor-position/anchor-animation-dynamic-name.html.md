# css/css-anchor-position/anchor-animation-dynamic-name.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-animation-dynamic-name.html"
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
  #a1, #a2 {
    width: 100px;
    height: 100px;
  }
  #a1 {
    background: tomato;
  }
  #a2 {
    background: seagreen;
  }
  .anchor { anchor-name: --a; }
  #anchored {
    width: 50px;
    height: 50px;
    background: skyblue;
    animation: --anim 1s steps(2, start);
    position: absolute;
    position-anchor: --a;
  }
  @keyframes --anim {
    from { top: anchor(top); }
    to { top: anchor(bottom); }
  }
```

```json
{
  "errors": 6,
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
