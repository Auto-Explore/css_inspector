# css/css-variables/variable-animation-substitute-within-keyframe-multiple.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-animation-substitute-within-keyframe-multiple.html"
}
```

## style[0]

```css

        @keyframes coloranim
        {
            from { color: blue; }
            to { --myColor: green; --endColor: var(--myColor); color: var(--endColor); }
        }

        /* Start the animation in the paused state and fill at both ends so we can inspect values from both keyframes. */
        #target
        {
            animation-name: coloranim;
            animation-duration: 1s;
            animation-play-state: paused;
            animation-fill-mode: both;
        }
        #target
        {
            color: red;
            --myColor: red;
            --endColor: red;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
