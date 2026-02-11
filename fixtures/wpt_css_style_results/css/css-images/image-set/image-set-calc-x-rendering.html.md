# css/css-images/image-set/image-set-calc-x-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-calc-x-rendering.html"
}
```

## style[0]

```css

  #test {
    background-image: image-set(url("/images/green.png") calc(0.5x * 2));
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
