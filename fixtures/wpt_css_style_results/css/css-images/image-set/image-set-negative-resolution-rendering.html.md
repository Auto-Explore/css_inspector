# css/css-images/image-set/image-set-negative-resolution-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-negative-resolution-rendering.html"
}
```

## style[0]

```css

  #test {
    background-image: url("/images/green.png");
    background-image: image-set(url("/images/red.png") -1x);
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
