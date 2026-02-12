# css/css-images/image-set/image-set-resolution-002.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-resolution-002.html"
}
```

## style[0]

```css

  body { margin: 0 }
  div {
    /* green.png is 100x50, should be 200x100 instead */
    background-image: -webkit-image-set(url('/images/green.png') 0.5x);
    background-image: image-set(url('/images/green.png') 0.5x);
    background-repeat: no-repeat;
    background-origin: 0 0;
    width: 100vw;
    height: 100vh;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
