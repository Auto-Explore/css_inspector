# css/css-animations/neutral-var-keyframe-cycle-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/neutral-var-keyframe-cycle-crash.html"
}
```

## style[0]

```css

    #test {
        animation: anim 1s;
    }

    @keyframes anim {
        to {
            --x: var(--x);
        }
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
