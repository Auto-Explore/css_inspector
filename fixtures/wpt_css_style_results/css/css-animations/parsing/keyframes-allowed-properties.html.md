# css/css-animations/parsing/keyframes-allowed-properties.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/parsing/keyframes-allowed-properties.html"
}
```

## style[0]

```css

@keyframes foo {
  from {
    /* Non-animation properties are allowed */
    margin-top: 10px;
    /* animation-timing-function is specially allowed */
    animation-timing-function: ease;
    /* All other animation properties are not allowed */
    animation-name: none;
    animation-duration: 1s;
    animation-iteration-count: 1;
    animation-direction: normal;
    animation-play-state: running;
    animation-delay: 0s;
    animation-fill-mode: none;
    /* The animation shorthand is also not allowed */
    animation: bar 1s infinite;
  }
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
