# css/css-writing-modes/abs-pos-replaced-vrl-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-replaced-vrl-001-ref.html"
}
```

## style[0]

```css

  .cb {
    position: relative;
  }
  audio, video, canvas, iframe, svg {
    border: 1px solid blue;
    position: absolute;
    /* This value corresponds to the difference between wrapper
       divs' width in the testcase. It causes the elements to
       be shifted right to prevent the test from spuriously
       passing by just placing them on the leftmost side. */
    left: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
