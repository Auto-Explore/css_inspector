# css/css-syntax/serialize-escape-identifiers.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/serialize-escape-identifiers.html"
}
```

## style[0]

```css

      @import 'abc' layer(\{\});
      @counter-style abc\{\}oops {}
      @font-feature-values abc\{\}oops {}
      @font-palette-values --abc\{\}oops {}
      @keyframes abc\{\}oops {}
      @layer abc\;oops\!;
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
