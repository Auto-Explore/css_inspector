# css/css-fonts/font-face-style-auto-static-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-style-auto-static-ref.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Lato";
        src: url('/fonts/Lato-Medium.ttf') format('truetype');
        font-display: swap;
        font-style: normal;
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
