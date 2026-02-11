# css/css-animations/neutral-var-keyframe-cycle-registered-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/neutral-var-keyframe-cycle-registered-crash.html"
}
```

## style[0]

```css

    @property --x {
        syntax: "<number>";
        initial-value: 0;
        inherits: false;
    }
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
