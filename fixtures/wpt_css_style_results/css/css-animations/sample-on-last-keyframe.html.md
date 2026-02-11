# css/css-animations/sample-on-last-keyframe.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/sample-on-last-keyframe.html"
}
```

## style[0]

```css

    #test {
      animation:  test 1s reverse paused;
    }
    @keyframes test {
      from { opacity: 0; }
      to { opacity: 0.5; }
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
