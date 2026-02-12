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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
