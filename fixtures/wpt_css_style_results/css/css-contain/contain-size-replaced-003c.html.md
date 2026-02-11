# css/css-contain/contain-size-replaced-003c.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-replaced-003c.html"
}
```

## style[0]

```css

  video, audio, canvas, svg, img, embed, object, iframe {
    border: 3px solid teal;
    contain: size;
    margin: 15px;
    height: max-content; /* This means 0, given 'contain:size'. */
    width: auto;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
