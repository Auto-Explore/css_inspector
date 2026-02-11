# css/css-page/crashtests/negative-margin-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/crashtests/negative-margin-print.html"
}
```

## style[0]

```css

  @page {
    margin: -10px;

    @bottom-right-corner {
      content: "";
      width: 30px;
      height: 30px;
      margin: 10px;
    }

    @right-middle {
      content: "";
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
