# css/css-images/image-set/image-set-all-options-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-all-options-invalid.html"
}
```

## style[0]

```css

  #target {
      width: 100px;
      height: 100px;
      box-sizing: border-box;
      background-color: red;
      border: 50px solid green;
      border-image: 1 / 10px image-set(url('data:image/png;base64,') type('image/unknown'));
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
