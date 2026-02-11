# css/css-mixins/mixin-from-import-with-media-queries.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-from-import-with-media-queries.html"
}
```

## style[0]

```css

      @import 'resources/imported-sheet-with-mixin.css' (width < 10000px);
      @import 'resources/imported-sheet-with-red-mixin.css' (width > 10000px);
      div {
        color: red;
        @apply --m1;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
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
