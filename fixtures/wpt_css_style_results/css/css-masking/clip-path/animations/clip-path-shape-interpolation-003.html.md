# css/css-masking/clip-path/animations/clip-path-shape-interpolation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-shape-interpolation-003.html"
}
```

## style[0]

```css

    @keyframes anim {
      from {
        clip-path: shape(nonzero from 20px 20px,
          hline by 60px, vline by 60px, hline by -60%, close,
          move to 30% 30px, hline by 40px, vline by 40px, hline by -40px, close);
      }
      to {
        clip-path: path(nonzero, "M50 50 h50 v50 h-50 z M20 20 h50 v50 h-50 z");
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
