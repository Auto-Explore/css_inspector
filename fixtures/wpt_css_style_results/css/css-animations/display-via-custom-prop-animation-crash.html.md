# css/css-animations/display-via-custom-prop-animation-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/display-via-custom-prop-animation-crash.html"
}
```

## style[0]

```css

  :root {
    --disp: block;
  }

  @keyframes switch {
    0% { --disp: none; }
    100% { --disp: block; }
  }

  #target {
    height: 100px;
    width: 100px;
    background-color: green;
    animation: switch 0.5s;
    display: var(--disp);
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
