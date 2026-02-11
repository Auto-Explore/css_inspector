# css/css-animations/animation-delay-006-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-006-manual.html"
}
```

## style[0]

```css

  #contatiner {
    animation-delay: 5s;
  }

  #test {
    animation-name: sample;
    animation-duration: 10s;
    animation-delay: initial;

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
