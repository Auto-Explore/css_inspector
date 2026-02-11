# css/filter-effects/css-filters-animation-blur.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/css-filters-animation-blur.html"
}
```

## style[0]

```css

        @keyframes animate {
            0% {
                filter: blur(20px);
            }
            100% {
                filter: none;
            }
        }

        .square {
            width: 100px;
            height: 100px;
            background: blue;
            animation-name: animate;
            animation-play-state: paused;
            animation-delay: -2s;
            animation-duration: 4s;
            animation-timing-function: linear;
        }
     
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
