# css/css-animations/animation-name-006-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-name-006-manual.html"
}
```

## style[0]

```css

  div::before {
    animation-name: sample;
    animation-duration: 10s;

    background-color: blue;
    content: "Filler Text";
    display: block;
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
