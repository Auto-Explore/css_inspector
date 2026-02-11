# css/css-images/gradient-content-box.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient-content-box.html"
}
```

## style[0]

```css

#x {
  background-origin: content-box;
  width: 200px;
  height: 200px;
  padding: 40px;
  background-image: repeating-linear-gradient(to bottom right, white, black, white 30px);
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
