# css/css-values/min-max-percentage-length-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/min-max-percentage-length-interpolation.html"
}
```

## style[0]

```css

@keyframes anim {
  from {
    width:  min(50px, 30%);
    height: min(75%, 160px);
  }
  to {
    width:  max(75%, 100px);
    height: max(50px, 20%);
  }
}

.test {
  background-color: green;
  animation: anim 1s linear;
  animation-delay: -.5s;
  animation-play-state: paused;
}

.fail {
  background: red;
  position: absolute;
  z-index: -1;
  width: 100px;
  height: 100px;
}

.container {
  position: absolute;
  width: 200px;
  height: 200px;
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
