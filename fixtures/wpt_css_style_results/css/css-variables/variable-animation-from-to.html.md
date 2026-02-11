# css/css-variables/variable-animation-from-to.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-animation-from-to.html"
}
```

## style[0]

```css

        @keyframes valueanim
        {
            from { --value: blue; }
            to { --value: green; }
        }

        /* Start the animation in the paused state and fill at both ends so we can inspect values from both keyframes. */
        #target
        {
            animation-name: valueanim;
            animation-duration: 1s;
            animation-play-state: paused;
            animation-fill-mode: both;
            --value: red;
            color: var(--value);
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
