# css/css-images/image-set/image-set-empty-url-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-empty-url-rendering.html"
}
```

## style[0]

```css

  #test {
    background-image: image-set(
      url("/images/green.png") 1x,
      url("") 2x
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
