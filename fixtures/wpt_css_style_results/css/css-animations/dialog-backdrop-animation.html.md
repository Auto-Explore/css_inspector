# css/css-animations/dialog-backdrop-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/dialog-backdrop-animation.html"
}
```

## style[0]

```css


dialog[open]::backdrop {
    animation: dialog-backdrop-animation 1ms;
}

@keyframes dialog-backdrop-animation {
    from { opacity: 0 }
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
