# css/css-backgrounds/background-clip-root.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip-root.html"
}
```

## style[0]

```css


html {
	padding: 20px;
	background: url('support/1x1-green.png'), red;
	background-clip: content-box, border-box;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-clip”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
