# css/css-variables/variable-animation-substitute-within-keyframe.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-animation-substitute-within-keyframe.html"
}
```

## style[0]

```css

        @keyframes coloranim
        {
            from { color: blue; }
            to { --endColor: green; color: var(--endColor); }
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
