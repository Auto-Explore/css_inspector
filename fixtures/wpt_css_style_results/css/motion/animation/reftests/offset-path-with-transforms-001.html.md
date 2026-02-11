# css/motion/animation/reftests/offset-path-with-transforms-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/animation/reftests/offset-path-with-transforms-001.html"
}
```

## style[0]

```css

      @keyframes anim {
        to {
          translate: 0px 100px;
          offset-distance: 100%;
          transform: translateX(-100px);
        }
      }
      #target {
        position: absolute;
        width: 100px;
        height: 50px;
        background-color: lime;
        offset-path: path("M25 0v100");
        animation: anim 10s -5s paused linear;
      }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-distance”.",
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
