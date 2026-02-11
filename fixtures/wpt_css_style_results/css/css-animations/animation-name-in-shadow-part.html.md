# css/css-animations/animation-name-in-shadow-part.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-name-in-shadow-part.html"
}
```

## style[0]

```css


#shadow {
  width: 100px;
  height: 100px;
}

@keyframes animation {
  from, to { background-color: green }
}

#shadow::part(target) {
  height: 100px;
  width: 100px;
  animation: animation 1s both;
  background-color: red;
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
      "message": "Invalid selector.",
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
