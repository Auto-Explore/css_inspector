# css/css-variables/variable-animation-over-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-animation-over-transition.html"
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
            transition-property: --value;
            transition-duration: 1s;
            --value: red;
            color: var(--value);
        }
        #target.changed
        {
            --value: black;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
