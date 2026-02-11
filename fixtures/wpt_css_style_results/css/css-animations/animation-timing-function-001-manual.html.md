# css/css-animations/animation-timing-function-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-timing-function-001-manual.html"
}
```

## style[0]

```css

  div {
    animation-name: sample;
    animation-duration: 10s;
    animation-timing-function: cubic-bezier(0,0,1,1);

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
