# css/css-ui/outline-018.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-018.html"
}
```

## style[0]

```css

@keyframes outline-anim {
  from {
    outline: solid 1px black;
  }
  to {
    outline: dotted 1px black;
  }
}

#test1 {
  animation: outline-anim 3s 0s paused linear;
  outline: solid 1px black;
}
#test2 {
  animation: outline-anim 3s -1s paused linear;
  outline: solid 1px black;
}
#test3 {
  animation: outline-anim 3s -2s paused linear;
  outline: solid 1px black;
}
#test4 {
  animation: outline-anim 3s 0s paused reverse linear;
  outline: solid 1px black;
}
```

```json
{
  "errors": 5,
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
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
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
