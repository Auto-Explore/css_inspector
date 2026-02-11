# css/css-backgrounds/background-attachment-fixed-border-radius-offset-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-attachment-fixed-border-radius-offset-ref.html"
}
```

## style[0]

```css

  .stripe-not-fixed {
    position: absolute;
    top: calc(8px - 128px);
    width: 200px;
    height: 683px;
    background-image: url("/images/grgr-256x256.png");
    background-position: 0px -80px;
    background-size: 1024px 768px;
    background-repeat: no-repeat;
    clip-path: inset(0 0 385px 0);
  }
  body {
    overflow: hidden;
    height: 300vh;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
