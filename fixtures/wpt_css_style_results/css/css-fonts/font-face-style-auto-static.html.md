# css/css-fonts/font-face-style-auto-static.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-style-auto-static.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Lato";
        src: url('/fonts/Lato-Medium.ttf') format('truetype');
        font-display: swap;
        font-style: auto;
    }
    .test {
        font-family: "Lato";
        font-size: 3em;
        font-style: oblique 10deg;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
