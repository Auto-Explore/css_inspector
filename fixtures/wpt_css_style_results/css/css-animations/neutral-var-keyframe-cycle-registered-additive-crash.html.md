# css/css-animations/neutral-var-keyframe-cycle-registered-additive-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/neutral-var-keyframe-cycle-registered-additive-crash.html"
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
        animation: anim1 100ms, anim2 100ms;
        animation-composition: replace, add;
    }

    @keyframes anim1 {
        from { --x: var(--x); }
        to { --x: var(--x); }
    }
    @keyframes anim2 {
        to { --x: var(--x); }
    }
```

```json
{
  "errors": 3,
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
      "message": "Unknown property “animation-composition”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
