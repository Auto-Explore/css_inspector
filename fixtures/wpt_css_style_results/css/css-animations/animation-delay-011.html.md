# css/css-animations/animation-delay-011.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-011.html"
}
```

## style[0]

```css

div:after {
  content: '';
  display: block;
  width: 100px;
  height: 100px;
  background: red;
  animation: doesntmatter 50s linear infinite,
             bg 100s step-end infinite;
  animation-play-state: paused;
  animation-delay: inherit;
}

@keyframes bg {
  50% { background: green; }
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
