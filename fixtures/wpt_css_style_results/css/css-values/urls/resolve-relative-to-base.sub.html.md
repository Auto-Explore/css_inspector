# css/css-values/urls/resolve-relative-to-base.sub.html

```json
{
  "format_version": 3,
  "file": "css/css-values/urls/resolve-relative-to-base.sub.html"
}
```

## style[0]

```css

:root {
    --image-path: url("images/test.png");
}
#relative-image-url {
    background-image: url(images/test.png);
}

#relative-image-variable-url {
    background-image: var(--image-path);
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
