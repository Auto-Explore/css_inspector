# css/css-backgrounds/background-attachment-local-inline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-attachment-local-inline-002.html"
}
```

## style[0]

```css

  body
    {
      color: transparent;
      font-family: Ahem;
      font-size: 50vh;
      height: 50vh;
      line-height: 2;
      margin: 0px;
    }

  span#test-passed
    {
      background-attachment: local;
      background-image: linear-gradient(to bottom, blue 25%, orange 25% 50%, lime 50% 75%, purple 75% 100%);
      vertical-align: top;
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
