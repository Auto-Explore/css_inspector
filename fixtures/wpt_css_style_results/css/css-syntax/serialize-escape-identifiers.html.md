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
  "errors": 2,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
