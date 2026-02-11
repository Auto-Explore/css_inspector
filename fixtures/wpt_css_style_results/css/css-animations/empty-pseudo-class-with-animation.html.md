# css/css-animations/empty-pseudo-class-with-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/empty-pseudo-class-with-animation.html"
}
```

## style[0]

```css


.container {
  width: 100px;
  height: 100px;
  background-color: rgb(0, 255, 0);
}

.container:empty {
  background-color: rgb(255, 0, 0);
  animation: anim 1s;
}

@keyframes anim { }

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
