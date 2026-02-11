# css/CSS2/syntax/case-sensitive-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/case-sensitive-004.xht"
}
```

## style[0]

```css

    .test-folding { color: red; }

    :lang(en) {
      color: green;
    }
    :lang(FR) {
      color: green;
    }

    /* test for ASCII (not UNICODE) case-insensitivity */
    .test-unicode { color: green; }
    :lang(fİ) {
      color: red;
    }
    :lang(Fı) {
      color: red;
    }
    :lang(&#x212a;l) {
      color: red;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
