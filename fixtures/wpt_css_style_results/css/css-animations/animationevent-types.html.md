# css/css-animations/animationevent-types.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animationevent-types.html"
}
```

## style[0]

```css

  #test {
    animation-name: sample;
    animation-duration: 2s;
    animation-delay: -1s;
    animation-iteration-count: 2;

    background-color: blue;
    height: 100px;
    width: 100px;
    position: relative;
  }

  @keyframes sample {
    from {
      left: 150px;
    }
    to {
      left: 0px;
    }
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
