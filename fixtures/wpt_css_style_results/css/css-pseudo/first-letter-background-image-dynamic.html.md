# css/css-pseudo/first-letter-background-image-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-background-image-dynamic.html"
}
```

## style[0]

```css

  div::first-letter {
    color: lime;
  }
  div.image::first-letter {
    /* Lime background */
    background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M/wHwAEBgIApD5fRAAAAABJRU5ErkJggg==');
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
