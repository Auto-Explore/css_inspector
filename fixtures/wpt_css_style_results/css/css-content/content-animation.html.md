# css/css-content/content-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-content/content-animation.html"
}
```

## style[0]

```css


.target::after {
  content: "default";
}

@keyframes content-animation {
  from { content: "from" }
  to   { content: "to" }
}

.target.animated::after {
  animation: content-animation 1s paused linear forwards;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
