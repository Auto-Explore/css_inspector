# css/css-variables/variable-animation-to-only.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-animation-to-only.html"
}
```

## style[0]

```css

        @keyframes valueanim
        {
            to { --value: green; }
        }

        /* Start the animation in the paused state and fill at both ends so we can inspect values from both keyframes. */
        #target
        {
            animation-name: valueanim;
            animation-duration: 1s;
            animation-play-state: paused;
            animation-fill-mode: both;
        }
        #target
        {
            --value: blue;
        }
        #target span
        {
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
