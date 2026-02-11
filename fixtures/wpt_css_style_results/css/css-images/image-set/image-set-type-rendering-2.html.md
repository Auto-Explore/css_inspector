# css/css-images/image-set/image-set-type-rendering-2.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-type-rendering-2.html"
}
```

## style[0]

```css

  #test {
    background-image: image-set(
      url("/images/green.png") type('image/png') 1x,
      url("/images/red.png") type('image/png') 2x
    );
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
