# css/css-pseudo/pseudo-replaced-elements.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/pseudo-replaced-elements.html"
}
```

## style[0]

```css

  input::before,
  video::before,
  progress::before {
    content: "X";
    display: block;
    /* Not resolvable if box is not generated */
    width: 10%;
  }
  span {
    display: block;
    /* Not resolvable if box is not generated */
    width: 10%;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
