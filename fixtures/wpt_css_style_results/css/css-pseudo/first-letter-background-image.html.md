# css/css-pseudo/first-letter-background-image.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-background-image.html"
}
```

## style[0]

```css

  div::first-letter {
    color: lime;
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
