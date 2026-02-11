# css/css-transforms/individual-transform/animation/individual-transform-ordering-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/individual-transform/animation/individual-transform-ordering-ref.html"
}
```

## style[0]

```css

      @keyframes anim {
        to {
          transform: translate(50px, 50px) rotate(45deg) scale(2, 1);
        }
      }
      .block {
        display:  inline-block;
        width: 50px;
        height: 50px;
        margin:  50px;
        padding:  0;
        transform-origin: 0 0;
        background:  lime;
        /* Freeze the animation at the midpoint. */
        animation-timing-function: cubic-bezier(0, 1, 1, 0);
        animation-duration:  1000000s;
        animation-delay:  -500000s;
        animation-name:  anim;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
