# css/css-animations/crashtests/keyframe-easing-var-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/crashtests/keyframe-easing-var-crash.html"
}
```

## style[0]

```css

@keyframes kf {
  0% {
    transform: translateX(0px);
    animation-timing-function: var(--kf_ease);
  }
  100% {
    transform: translateX(100px);
  }
}

#target {
  width: 40px;
  height: 40px;
  background: #4a90e2;
  animation: kf 1s linear 1;
  will-change: transform;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
