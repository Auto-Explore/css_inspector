# css/css-transitions/transition-after-animation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/transition-after-animation-001.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { left: 100px }
    to { left: 200px }
  }
  #div {
    left: 0px;
    position: relative;
    width: 50px;
    height: 50px;
    background-color: green;
  }
  #div.animate {
    animation: anim 0.1s linear;
  }
  #div.transition {
    left: 300px;
    transition: left 1000s steps(2, start);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
