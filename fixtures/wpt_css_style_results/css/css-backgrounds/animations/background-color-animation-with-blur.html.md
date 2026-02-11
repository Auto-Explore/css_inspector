# css/css-backgrounds/animations/background-color-animation-with-blur.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-with-blur.html"
}
```

## style[0]

```css

  @keyframes colorize {
    0%   { background-color:  blue; }
    50%  { background-color: green; }
    100% { background-color:  red; }
  }
  #target {
    height: 100px;
    width:  100px;
    animation-name: colorize;
    animation-duration: 100000s;
    animation-timing-function: steps(2, end);
    display:  inline-block;
    margin:  10px;
  }
  .blur {
    filter:  blur(5px);
  }
  .color-shift {
    animation-delay: -50000s;
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
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
