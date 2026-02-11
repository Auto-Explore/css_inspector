# css/css-animations/parent-after-change-style-pseudo-element-flicker.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/parent-after-change-style-pseudo-element-flicker.html"
}
```

## style[0]

```css

div {
    position: absolute;
    width: 100px;
    height: 100px;
}

@keyframes animation {
    from, to { margin-left: 0 }
}

@starting-style {
    .parent:after { }
}

.parent {
    animation: animation 10000s;
}

.child:after {
    content: "";
    position: absolute;
    inset: 0;
    background-color: black;
    animation: animation 10000s;
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
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
