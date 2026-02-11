# css/css-images/image-set/image-set-content-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-content-rendering.html"
}
```

## style[0]

```css

  #test {
    content: image-set(url("/images/green.png") 1x);
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
