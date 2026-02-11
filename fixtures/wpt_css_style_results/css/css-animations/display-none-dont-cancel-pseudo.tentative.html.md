# css/css-animations/display-none-dont-cancel-pseudo.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/display-none-dont-cancel-pseudo.tentative.html"
}
```

## style[0]

```css


@keyframes display-animation {
  from { margin-left: 100px; display: block }
  to   { margin-left: 200px; display: none }
}

.target::after {
  content: "";
  margin-left: 50px;
}

.target.animated::after {
  animation: display-animation 1s forwards;
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
