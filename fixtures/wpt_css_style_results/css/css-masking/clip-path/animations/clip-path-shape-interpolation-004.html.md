# css/css-masking/clip-path/animations/clip-path-shape-interpolation-004.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-shape-interpolation-004.html"
}
```

## style[0]

```css

    @keyframes anim {
      from {
        clip-path: path(evenodd, "M20 20 h60 v60 h-60 z M30 30 h40 v40 h-40 z");
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
