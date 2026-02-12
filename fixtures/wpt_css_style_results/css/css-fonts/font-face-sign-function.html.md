# css/css-fonts/font-face-sign-function.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-sign-function.html"
}
```

## style[0]

```css

  :root {
    font-size: 16px;
  }
  @font-face {
    font-family: "Lato";
    src: url('/fonts/Lato-Medium.ttf') format('truetype');
    font-display: swap;
    font-weight: calc(sign(1rem - 1px) * 400);
    font-width: calc(sign(1rem - 1px) * 100%);
  }

  .test {
    font-family: "Lato";
    font-size: 3em;
    font-weight: bold;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
