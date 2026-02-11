# css/css-images/image-set/image-set-dppx-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-dppx-rendering.html"
}
```

## style[0]

```css

  #test {
    background-image: image-set(url("/images/green.png") 1dppx);
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
