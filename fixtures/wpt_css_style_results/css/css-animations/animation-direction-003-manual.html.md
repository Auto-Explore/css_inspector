# css/css-animations/animation-direction-003-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-direction-003-manual.html"
}
```

## style[0]

```css

  div {
    animation-name: sample;
    animation-duration: 10s;
    animation-direction: alternate-reverse;
    animation-iteration-count: infinite;

    background-color: blue;
    height: 100px;
    width: 100px;
    position: relative;
  }

  @keyframes sample {
    from {
      left: 0px;
    }
    to {
      left: 150px;
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
