# css/css-masking/clip-path/animations/clip-path-animation-retarget.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-retarget.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  clip-path: inset(20% 20%);
}

#animinitial {
  background-color: green;
}

#animfinal {
  background-color: green;
}

@keyframes clippath {
  0%   { clip-path: circle(50% at 50% 50%); }
  100% { clip-path: circle(20% at 20% 20%); }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
