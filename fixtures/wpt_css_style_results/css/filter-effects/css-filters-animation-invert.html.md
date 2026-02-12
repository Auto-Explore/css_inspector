# css/filter-effects/css-filters-animation-invert.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/css-filters-animation-invert.html"
}
```

## style[0]

```css

        @keyframes animate {
            0% {
                filter: invert(100%);
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
