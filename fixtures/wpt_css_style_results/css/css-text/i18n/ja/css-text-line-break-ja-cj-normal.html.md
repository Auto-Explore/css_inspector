# css/css-text/i18n/ja/css-text-line-break-ja-cj-normal.html

```json
{
  "format_version": 3,
  "file": "css/css-text/i18n/ja/css-text-line-break-ja-cj-normal.html"
}
```

## style[0]

```css

@font-face {
    font-family: 'mplus-1p-regular';
    src: url('/fonts/mplus-1p-regular.woff') format('woff');
    }
#wrapper { position: relative; }
.test { color: red; }
.test, .ref { font-size: 30px; font-family: mplus-1p-regular, sans-serif; width: 185px; padding: 0; border: 1px solid orange; line-height: 1em; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

.test { line-break: normal; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
