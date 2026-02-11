# css/css-animations/animation-delay-007-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-007-manual.html"
}
```

## style[0]

```css

  #container {
    animation-delay: 5s;
  }

  #test {
    animation-name: sample;
    animation-duration: 5s;
    animation-delay: inherit;

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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
