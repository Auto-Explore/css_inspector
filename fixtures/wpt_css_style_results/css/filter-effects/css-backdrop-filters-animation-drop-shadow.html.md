# css/filter-effects/css-backdrop-filters-animation-drop-shadow.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/css-backdrop-filters-animation-drop-shadow.html"
}
```

## style[0]

```css

    @keyframes animate {
        0% {
            backdrop-filter: drop-shadow(30px 30px 0px black);
        }
        100% {
            backdrop-filter: none;
        }
    }
    .square {
        position: absolute;
        width: 100px;
        height: 100px;
        top: 100px;
        left: 100px;
        background: blue;
    }
    .filt {
        position: absolute;
        width: 200px;
        height: 200px;
        top: 50px;
        left: 50px;
        animation-name: animate;
        animation-play-state: paused;
        animation-delay: -2s;
        animation-duration: 4s;
        animation-timing-function: linear;
    }
    .greenbox {
        position: absolute;
        width: 50px;
        height: 50px;
        top: 75px;
        left: 75px;
        background: green;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
