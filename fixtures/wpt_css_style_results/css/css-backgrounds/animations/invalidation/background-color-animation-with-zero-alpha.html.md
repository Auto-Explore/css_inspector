# css/css-backgrounds/animations/invalidation/background-color-animation-with-zero-alpha.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/invalidation/background-color-animation-with-zero-alpha.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  /* Use a long animation that start at 50% progress where the slope of the
     selected timing function is zero. By setting up the animation in this way,
     we accommodate lengthy delays in running the test without a potential drift
     in the animated property value. This is important for avoiding flakes,
     especially on debug builds. The screenshots are taken as soon as the
     animation is ready, thus the long animation duration has no bearing on
     the actual duration of the test. */
  animation: bgcolor 1000000s cubic-bezier(0,1,1,0) -500000s;
}
@keyframes bgcolor {
  0% { background-color: rgb(0, 200, 0, 0); }
  100% { background-color: rgb(200, 0, 0, 0); }
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
