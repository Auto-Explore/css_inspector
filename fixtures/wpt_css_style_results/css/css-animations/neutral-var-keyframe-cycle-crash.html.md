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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
