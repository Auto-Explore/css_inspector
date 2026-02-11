# css/motion/animation/reftests/offset-rotate-interpolation-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/animation/reftests/offset-rotate-interpolation-001.html"
}
```

## style[0]

```css

      @keyframes anim {
        from { offset-rotate: auto; }
        to   { offset-rotate: reverse; }
      }
      #target {
        position: absolute;
        left: 300px;
        top: 0px;
        width: 300px;
        height: 200px;
        background-color: lime;
        transform-origin: 0px 0px;
        offset-path: path("m 100 150 v -100");
        animation: anim 10s -5s paused linear;
      }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “offset-rotate”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-rotate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
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
