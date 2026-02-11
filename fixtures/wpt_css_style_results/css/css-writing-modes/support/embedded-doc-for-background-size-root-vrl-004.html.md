# css/css-writing-modes/support/embedded-doc-for-background-size-root-vrl-004.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/support/embedded-doc-for-background-size-root-vrl-004.html"
}
```

## style[0]

```css

    html
      {
        background-image: url("swatch-green.png");
        background-repeat: no-repeat;
        background-size: 100% 100%;
        writing-mode: vertical-rl;

        border-right: green solid 20px;
        width: 80px;
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
