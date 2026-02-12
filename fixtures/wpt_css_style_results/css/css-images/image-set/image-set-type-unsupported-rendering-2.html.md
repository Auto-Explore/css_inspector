# css/css-images/image-set/image-set-type-unsupported-rendering-2.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-type-unsupported-rendering-2.html"
}
```

## style[0]

```css

  #test {
    background-image: url("/images/red.png");
    background-image: image-set(
      url("/images/green.png") 1x type('image/unsupported'),
      url("/images/green.png") 1x type('image/unsupported')
    );
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
