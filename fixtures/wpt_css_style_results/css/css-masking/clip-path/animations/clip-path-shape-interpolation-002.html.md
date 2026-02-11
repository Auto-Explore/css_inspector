# css/css-masking/clip-path/animations/clip-path-shape-interpolation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-shape-interpolation-002.html"
}
```

## style[0]

```css

    @keyframes anim {
      from {
        clip-path: shape(evenodd from 20px 20px,
          hline by 60px, vline by 60px, hline by -60%, close,
          move to 30% 30px, hline by 40px, vline by 40px, hline by -40px, close);
      }
      to {
        clip-path: shape(evenodd from 50px 50px,
          hline by 50px, vline by 50px, hline by -50%, close,
          move to 20px 20%, hline by 50px, vline by 50px, hline by -50px, close);
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
