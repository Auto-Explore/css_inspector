# css/css-transforms/animation/scale-and-rotate-both-specified-on-animation-keyframes.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/scale-and-rotate-both-specified-on-animation-keyframes.html"
}
```

## style[0]

```css


@keyframes scale-and-rotate-animation {
    0%      { scale: 1.5; rotate: 0deg; }
    0.001%  { scale: 1; rotate: 90deg; }
    100%    { scale: 1; rotate: 90deg; }
}

#target {
    position: absolute;
    width: 100px;
    height: 100px;
    background-color: black;

    transform-origin: bottom left;
    animation: scale-and-rotate-animation linear 100s;
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
      "message": "Invalid value for property “transform-origin”.",
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
