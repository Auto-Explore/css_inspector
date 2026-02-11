# css/filter-effects/css-filters-animation-combined-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/css-filters-animation-combined-001.html"
}
```

## style[0]

```css

        @keyframes animate {
            0% {
                filter:
                    blur(20px)
                    brightness(0%)
                    sepia(100%)
                    grayscale(100%)
                    saturate(0%)
                    hue-rotate(90deg)
                    invert(100%)
                    opacity(0%)
            }
            100% {
                filter: none;
            }
        }

        .square {
            width: 100px;
            height: 100px;
            background: black;
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
