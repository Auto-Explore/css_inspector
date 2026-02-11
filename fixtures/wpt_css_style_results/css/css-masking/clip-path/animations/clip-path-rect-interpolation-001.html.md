# css/css-masking/clip-path/animations/clip-path-rect-interpolation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-rect-interpolation-001.html"
}
```

## style[0]

```css

    @keyframes anim {
      from {
        clip-path: rect(0px 100px 100% 0px round 10px);
      }
      to {
        clip-path: rect(20px 80px calc(20px + 60%) 20px round 20px);
      }
    }
    #rect {
      width: 100px;
      height: 100px;
      background-color: green;
      animation: anim 10s -5s paused linear;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
