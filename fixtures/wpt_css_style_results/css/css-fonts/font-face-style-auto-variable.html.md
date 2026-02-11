# css/css-fonts/font-face-style-auto-variable.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-style-auto-variable.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Inter";
        src: url('support/fonts/Inter-VF.subset.ttf') format('truetype');
        font-display: swap;
        font-style: auto;
    }
    .style10 {
        font-family: "Inter";
        font-size: 3em;
        font-style: oblique 10deg;
    }
    .style5 {
        font-family: "Inter";
        font-size: 3em;
        font-style: oblique 5deg;
    }
    .style0 {
        font-family: "Inter";
        font-size: 3em;
        font-style: oblique 0deg;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
