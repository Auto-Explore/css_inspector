# css/css-page/background-image-only-for-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/background-image-only-for-print.html"
}
```

## style[0]

```css

  @media not print {
    .print-only {
      display: none;
    }
  }

  :root {
    print-color-adjust: exact;
  }

  #target {
    background-image: url("/images/green.png");
    height: 50px;
    width: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
