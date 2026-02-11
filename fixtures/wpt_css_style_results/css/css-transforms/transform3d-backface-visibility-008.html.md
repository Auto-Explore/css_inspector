# css/css-transforms/transform3d-backface-visibility-008.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform3d-backface-visibility-008.html"
}
```

## style[0]

```css

  div {
    height: 100px;
    width: 100px;
  }

  body > div {
    background: red;
  }

  div > div {
    background: green;
    backface-visibility: visible;
    transform: rotateY(180deg);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
